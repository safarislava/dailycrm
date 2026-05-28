use crate::common::BoxError;
use crate::contract::sting_contentable::StringContentable;

#[derive(Clone)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Password {
        Password(password)
    }
}

#[async_trait::async_trait]
impl StringContentable for Password {
    async fn content(&self) -> Result<String, BoxError> {
        Ok(self.0.clone())
    }
}
