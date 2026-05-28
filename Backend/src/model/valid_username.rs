use crate::common::BoxError;
use crate::contract::sting_contentable::StringContentable;
use crate::model::username::Username;

pub struct ValidUsername(Username);

impl ValidUsername {
    pub fn new(username: Username) -> ValidUsername {
        Self(username)
    }
}

impl StringContentable for ValidUsername {
    fn content(&self) -> Result<String, BoxError> {
        let content = self.0.content()?;
        let len = content.len();
        if len < 3 {
            return Err(Box::new(&UsernameError::TooShort));
        }
        if len > 50 {
            return Err(Box::new(&UsernameError::TooLong));
        }
        if !content.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            return Err(Box::new(&UsernameError::InvalidChars));
        }
        Ok(content)
    }
}

#[derive(Debug)]
pub enum UsernameError {
    TooShort,
    TooLong,
    InvalidChars,
}

impl std::fmt::Display for UsernameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            Self::TooShort | Self::TooLong => "Username must be 3–50 characters",
            Self::InvalidChars => "Username may only contain letters, digits, _ or -",
        })
    }
}

impl std::error::Error for UsernameError {}
