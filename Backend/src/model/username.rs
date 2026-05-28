use crate::common::BoxError;
use crate::contract::sting_contentable::StringContentable;

pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Self {
        Self(username)
    }
}

impl StringContentable for Username {
    fn content(&self) -> Result<String, BoxError> {
        Ok(self.0.clone())
    }
}
