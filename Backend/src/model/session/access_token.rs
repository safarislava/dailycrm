use crate::common::BoxError;
use crate::model::session::contract::token::Token;

pub struct AccessToken {
    token: Box<dyn Token>,
}

impl AccessToken {
    pub fn new(token: Box<dyn Token>) -> Self {
        AccessToken { token }
    }
}

#[async_trait::async_trait]
impl Token for AccessToken {
    async fn value(&self) -> Result<String, BoxError> {
        self.token.value().await
    }
}