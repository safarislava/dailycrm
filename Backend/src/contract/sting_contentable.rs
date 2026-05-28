use crate::common::BoxError;

#[async_trait::async_trait]
pub trait StringContentable {
    async fn content(&self) -> Result<String, BoxError>;
}