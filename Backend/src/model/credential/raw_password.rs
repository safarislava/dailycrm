use crate::common::BoxError;
use crate::model::credential::contract::password::Password;

#[derive(Clone)]
pub struct RawPassword(String);

impl RawPassword {
    pub fn new(password: String) -> Self {
        Self(password)
    }
}

impl Password for RawPassword {
    fn value(&self) -> Result<String, BoxError> {
        Ok(self.0.clone())
    }
}
