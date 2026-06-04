use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Token: Send + Sync {
    async fn value(&self) -> Result<String, BoxError>;
}

#[async_trait::async_trait]
impl Token for String {
    async fn value(&self) -> Result<String, BoxError> {
        Ok(self.clone())
    }
}