use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Json: Send + Sync {
    async fn json(&self) -> Result<serde_json::Value, BoxError>;
}