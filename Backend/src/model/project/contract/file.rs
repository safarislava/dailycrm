use crate::common::BoxError;
use crate::storage::Storage;

#[async_trait::async_trait]
pub trait File: Send + Sync {
    fn name(&self) -> &str;
    fn media_type(&self) -> &str;
    fn size_bytes(&self) -> i64;
    async fn upload_to(&self, storage: &Storage, key: &str) -> Result<(), BoxError>;
}