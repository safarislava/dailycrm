use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Scheduled: Send + Sync {
    async fn run(&self) -> Result<(), BoxError>;
}
