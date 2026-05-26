pub struct Username(pub String);

pub struct ValidUsername(Username);

impl ValidUsername {
    pub fn try_new(username: Username) -> Result<Self, UsernameError> {
        let len = username.0.len();
        if len < 3 {
            return Err(UsernameError::TooShort);
        }
        if len > 50 {
            return Err(UsernameError::TooLong);
        }
        if !username
            .0
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(UsernameError::InvalidChars);
        }
        Ok(Self(username))
    }

    pub fn as_str(&self) -> &str {
        &self.0.0
    }
}

pub enum UsernameError {
    TooShort,
    TooLong,
    InvalidChars,
}

impl UsernameError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::TooShort | Self::TooLong => "Username must be 3–50 characters",
            Self::InvalidChars => "Username may only contain letters, digits, _ or -",
        }
    }
}
