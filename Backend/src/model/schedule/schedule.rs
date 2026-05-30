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
            self.event.fired().await?;
            self.task.done().await?;
        }
    }
}