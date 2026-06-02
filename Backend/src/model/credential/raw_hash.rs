use crate::common::BoxError;
use crate::model::credential::contract::hash::Hash;

#[derive(Clone)]
pub struct RawHash(String);

impl RawHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }
}

#[async_trait::async_trait]
impl Hash for RawHash {
    async fn value(&self) -> Result<String, BoxError> {
        Ok(self.0.clone())
    }
}
