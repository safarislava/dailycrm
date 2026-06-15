use crate::model::credential::contract::password::{Password, PasswordError};

pub struct ValidPassword(Box<dyn Password>);

impl ValidPassword {
    pub fn new(password: impl Password) -> Self {
        Self(Box::new(password))
    }
}

impl Password for ValidPassword {
    fn value(&self) -> Result<String, PasswordError> {
        let content = self.0.value()?;
        let len = content.len();
        if len < 6 {
            return Err(PasswordError::TooShort);
        }
        if len > 72 {
            return Err(PasswordError::TooLong);
        }
        Ok(content)
    }
}
