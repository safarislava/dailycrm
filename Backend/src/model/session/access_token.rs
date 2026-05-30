use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
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
impl Contentable for AccessToken {
    type Output = String;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        Ok(self.token.content().await?)
    }
}

impl Token for AccessToken {}
