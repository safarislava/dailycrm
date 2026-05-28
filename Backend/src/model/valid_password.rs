use crate::common::BoxError;
use crate::contract::contentable::Contentable;
use crate::model::password::Password;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct ValidPassword(Password);

impl ValidPassword {
    pub fn new(password: Password) -> Self {
        ValidPassword(password)
    }
}

#[async_trait::async_trait]
impl Contentable for ValidPassword {
    type Output = String;
    async fn content(&self) -> Result<String, BoxError> {
        let content = self.0.content().await?;
        let len = content.len();
        if len < 8 {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::TooShort | Self::TooLong => "Password must be 8–72 characters",
        })
    }
}

impl std::error::Error for PasswordError {}
