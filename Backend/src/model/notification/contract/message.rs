use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Message: Send + Sync {
    async fn text(&self) -> Result<String, BoxError>;
}