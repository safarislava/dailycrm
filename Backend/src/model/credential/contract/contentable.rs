use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Contentable {
    type Output;
    async fn content(&self) -> Result<Self::Output, BoxError>;
}
