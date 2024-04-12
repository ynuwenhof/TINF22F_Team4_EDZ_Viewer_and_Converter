use std::collections::VecDeque;
use std::fs::File;
use time::OffsetDateTime;
use tokio::sync::oneshot;

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
