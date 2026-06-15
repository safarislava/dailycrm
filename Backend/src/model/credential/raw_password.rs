use crate::model::credential::contract::password::{Password, PasswordError};

#[derive(Clone)]
pub struct RawPassword(String);

impl RawPassword {
    pub fn new(password: String) -> Self {
        Self(password)
    }
}

impl Password for RawPassword {
    fn value(&self) -> Result<String, PasswordError> {
        Ok(self.0.clone())
    }
}
