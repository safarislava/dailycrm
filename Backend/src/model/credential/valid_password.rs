use crate::common::BoxError;
use crate::model::credential::contract::password::Password;

pub struct ValidPassword(Box<dyn Password>);

impl ValidPassword {
    pub fn new(password: impl Password) -> Self {
        Self(Box::new(password))
    }
}

impl Password for ValidPassword {
    fn value(&self) -> Result<String, BoxError> {
        let content = self.0.value()?;
        let len = content.len();
        if len < 6 {
            return Err(Box::new(PasswordError::TooShort));
        }
        if len > 72 {
            return Err(Box::new(PasswordError::TooLong));
        }
        Ok(content)
    }
}

#[derive(Debug)]
pub enum PasswordError {
    TooShort,
    TooLong,
}

impl std::fmt::Display for PasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::TooShort | Self::TooLong => "Password must be 6–72 characters",
        })
    }
}

impl std::error::Error for PasswordError {}