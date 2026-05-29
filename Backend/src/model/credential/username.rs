use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;

pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Self {
        Self(username)
    }
}

#[async_trait::async_trait]
impl Contentable for Username {
    type Output = String;
    async fn content(&self) -> Result<String, BoxError> {
        Ok(self.0.clone())
    }
}
