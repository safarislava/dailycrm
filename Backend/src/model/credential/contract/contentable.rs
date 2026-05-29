use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Contentable: Send + Sync {
    type Output;
    async fn content(&self) -> Result<Self::Output, BoxError>;
}
