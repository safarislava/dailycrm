use crate::model::credential::contract::username::{Username, UsernameError};

pub struct RawUsername(String);

impl RawUsername {
    pub fn new(username: String) -> Self {
        Self(username)
    }
}

impl Username for RawUsername {
    fn value(&self) -> Result<String, UsernameError> {
        Ok(self.0.clone())
    }
}
