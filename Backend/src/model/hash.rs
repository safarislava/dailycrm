use crate::common::BoxError;
use crate::contract::sting_contentable::StringContentable;

#[derive(Clone)]
pub struct Hash(String);

impl Hash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }
}

#[async_trait::async_trait]
impl StringContentable for Hash {
    async fn content(&self) -> Result<String, BoxError> {
        Ok(self.0.clone())
    }
}

pub enum HashError {
    Bcrypt,
    Task,
}
