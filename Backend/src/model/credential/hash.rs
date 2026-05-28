use crate::common::BoxError;
use crate::contract::contentable::Contentable;

#[derive(Clone)]
pub struct Hash(String);

impl Hash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }
}

#[async_trait::async_trait]
impl Contentable for Hash {
    type Output = String;

    async fn content(&self) -> Result<String, BoxError> {
        Ok(self.0.clone())
    }
}
