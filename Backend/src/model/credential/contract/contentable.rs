use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Contentable: Send + Sync {
    type Output;
    async fn content(&self) -> Result<Self::Output, BoxError>;
}

#[async_trait::async_trait]
impl Contentable for String {
    type Output = String;
    async fn content(&self) -> Result<Self::Output, BoxError> {
        Ok(self.clone())
    }
}
