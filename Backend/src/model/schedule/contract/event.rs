use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Event: Send + Sync {
    async fn fired(&self) -> Result<(), BoxError>;
}
