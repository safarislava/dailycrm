use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Hash: Send + Sync + 'static {
    async fn value(&self) -> Result<String, BoxError>;
}