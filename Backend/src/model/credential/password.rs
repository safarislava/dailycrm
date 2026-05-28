use crate::common::BoxError;
use crate::contract::contentable::Contentable;

#[derive(Clone)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Password {
        Password(password)
    }
}

#[async_trait::async_trait]
impl Contentable for Password {
    type Output = String;
    async fn content(&self) -> Result<String, BoxError> {
        Ok(self.0.clone())
    }
}
