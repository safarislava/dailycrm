use crate::common::BoxError;
use crate::model::schedule::contract::event::Event;
use crate::model::schedule::contract::scheduled::Scheduled;
use crate::model::task::contract::task::Task;
use std::sync::Arc;

pub struct Schedule {
    event: Arc<dyn Event>,
    task: Arc<dyn Task<Output = ()> + Send + Sync>,
}

impl Schedule {
    pub fn new(event: Arc<dyn Event>, task: Arc<dyn Task<Output = ()> + Send + Sync>) -> Self {
        Self { event, task }
    }
}

#[async_trait::async_trait]
impl Scheduled for Schedule {
    async fn run(&self) -> Result<(), BoxError> {
        loop {
            if let Err(error) = self.event.fired().await {
                eprintln!("schedule event error: {error}");
                continue;
            }
            if let Err(error) = self.task.done().await {
                eprintln!("schedule task error: {error}");
            }
        }
    }
}
