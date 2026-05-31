use crate::common::BoxError;
use crate::model::schedule::contract::event::Event;
use std::time::Duration;

pub struct PollInterval {
    duration: Duration,
}

impl PollInterval {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

#[async_trait::async_trait]
impl Event for PollInterval {
    async fn fired(&self) -> Result<(), BoxError> {
        actix_web::rt::time::sleep(self.duration).await;
        Ok(())
    }
}
