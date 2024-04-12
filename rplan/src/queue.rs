use super::error;
use std::collections::VecDeque;
use std::fs::File;
use time::OffsetDateTime;
use tokio::sync::{mpsc, oneshot};
use tokio::task;

pub struct Queue {
    tx: mpsc::Sender<Message>,
}

impl Queue {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(8);

        task::spawn(task(rx));

        Self { tx }
    }

    pub async fn get_task(&self, task_hash: String) -> Option<Task> {
        let (tx, rx) = oneshot::channel();
        let msg = Message::GetTask(task_hash, tx);
        let _ = self.tx.send(msg).await;
        rx.await.ok().flatten()
    }

    pub async fn push_entry(&self, entry: Entry) -> error::Result<Task> {
        let (tx, rx) = oneshot::channel();
        let msg = Message::PushEntry(entry, tx);
        let _ = self.tx.send(msg).await;
        Ok(rx.await?)
    }

    pub async fn push_analyzer(&self) -> error::Result<Entry> {
        let (tx, rx) = oneshot::channel();
        let msg = Message::PushAnalyzer(tx);
        let _ = self.tx.send(msg).await;
        Ok(rx.await?)
    }

    pub async fn remove_task(&self, task_hash: String) {
        let msg = Message::RemoveTask(task_hash);
        let _ = self.tx.send(msg).await;
    }

    pub async fn set_status(&self, task_hash: String, status: Status) {
        let msg = Message::SetStatus(task_hash, status);
        let _ = self.tx.send(msg).await;
    }
}

pub struct Entry {
    pub task: Task,
    pub file: File,
}

#[derive(Clone)]
pub struct Task {
    pub status: Status,
    pub hash: String,
    pub name: String,
    pub size: usize,
    pub created: OffsetDateTime,
}

#[derive(Clone)]
pub enum Status {
    Pending,
    Running,
    Failed,
}

async fn task(mut rx: mpsc::Receiver<Message>) {
    let mut queue = QueueInner::default();

    while let Some(msg) = rx.recv().await {
        match msg {
            Message::GetTask(task_hash, tx) => {
                let task = queue.get_task(&task_hash);
                let _ = tx.send(task.cloned());
            }
            Message::PushEntry(entry, tx) => {
                let task = queue.push_entry(entry);
                let _ = tx.send(task);
            }
            Message::PushAnalyzer(tx) => queue.push_analyzer(tx),
            Message::RemoveTask(task_hash) => queue.remove_task(&task_hash),
            Message::SetStatus(task_hash, status) => queue.set_status(&task_hash, status),
        }
    }
}

#[derive(Default)]
struct QueueInner {
    queue: VecDeque<Entry>,
    tasks: Vec<Task>,
    analyzers: VecDeque<oneshot::Sender<Entry>>,
}

impl QueueInner {
    fn get_task(&self, task_hash: &str) -> Option<&Task> {
        self.queue
            .iter()
            .find(|e| e.task.hash == task_hash)
            .map(|e| &e.task)
            .or_else(|| self.tasks.iter().find(|t| t.hash == task_hash))
    }

    fn push_entry(&mut self, entry: Entry) -> Task {
        let pre_existing_task = self.get_task(&entry.task.hash);
        if let Some(task) = pre_existing_task {
            return task.clone();
        }

        let mut task = entry.task.clone();
        match self.analyzers.pop_front() {
            None => self.queue.push_back(entry),
            Some(analyzer) => {
                task.status = Status::Running;
                self.tasks.push(task.clone());
                let _ = analyzer.send(entry);
            }
        }

        task
    }

    fn push_analyzer(&mut self, analyzer: oneshot::Sender<Entry>) {
        match self.queue.pop_front() {
            None => self.analyzers.push_back(analyzer),
            Some(mut entry) => {
                entry.task.status = Status::Running;
                self.tasks.push(entry.task.clone());
                let _ = analyzer.send(entry);
            }
        }
    }

    fn remove_task(&mut self, task_hash: &str) {
        let index = self.tasks.iter().position(|t| t.hash == task_hash);
        if let Some(index) = index {
            self.tasks.remove(index);
        }
    }

    fn set_status(&mut self, task_hash: &str, status: Status) {
        let task = self.tasks.iter_mut().find(|t| t.hash == task_hash);
        if let Some(task) = task {
            task.status = status;
        }
    }
}

enum Message {
    GetTask(String, oneshot::Sender<Option<Task>>),
    PushEntry(Entry, oneshot::Sender<Task>),
    PushAnalyzer(oneshot::Sender<Entry>),
    RemoveTask(String),
    SetStatus(String, Status),
}
