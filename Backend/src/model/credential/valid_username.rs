use crate::common::BoxError;
use crate::model::credential::contract::username::Username;

pub struct ValidUsername(Box<dyn Username>);

impl ValidUsername {
    pub fn new(username: impl Username) -> Self {
        Self(Box::new(username))
    }
}

impl Username for ValidUsername {
    fn value(&self) -> Result<String, BoxError> {
        let content = self.0.value()?;
        let len = content.chars().count();
        if len < 3 {
            return Err(Box::new(UsernameError::TooShort));
        }
        if len > 50 {
            return Err(Box::new(UsernameError::TooLong));
        }
        if !content.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || c == '_'
                || c == '-'
                || c == ' '
                || ('\u{0400}'..='\u{04FF}').contains(&c)
        }) {
            return Err(Box::new(UsernameError::InvalidChars));
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
            Self::InvalidChars => "Username may only contain letters, digits, spaces, _ or -",
        })
    }
}

impl std::error::Error for UsernameError {}