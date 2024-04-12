use super::db::Database;
use super::queue::Queue;
use super::Cli;
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub cli: Arc<Cli>,
    pub queue: Arc<Queue>,
    pub db: Arc<Database>,
}

impl Context {
    pub fn new(cli: Cli, db: Database) -> Self {
        let queue = Queue::new();

        Self {
            cli: Arc::new(cli),
            queue: Arc::new(queue),
            db: Arc::new(db),
        }
    }
}
