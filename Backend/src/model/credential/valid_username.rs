use crate::model::credential::contract::username::{Username, UsernameError};

pub struct ValidUsername(Box<dyn Username>);

impl ValidUsername {
    pub fn new(username: impl Username) -> Self {
        Self(Box::new(username))
    }
}

impl Username for ValidUsername {
    fn value(&self) -> Result<String, UsernameError> {
        let content = self.0.value()?;
        let len = content.chars().count();
        if len < 3 {
            return Err(UsernameError::TooShort);
        }
        if len > 50 {
            return Err(UsernameError::TooLong);
        }
        if !content.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || c == '_'
                || c == '-'
                || c == ' '
                || ('\u{0400}'..='\u{04FF}').contains(&c)
        }) {
            return Err(UsernameError::InvalidChars);
        }
        Ok(content)
    }
}
