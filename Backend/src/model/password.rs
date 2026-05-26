pub struct Password(pub String);
pub struct ValidPassword(Password);

impl ValidPassword {
    pub fn try_new(password: Password) -> Result<Self, PasswordError> {
        let len = password.0.len();
        if len < 8 {
            return Err(PasswordError::TooShort);
        }
        if len > 72 {
            return Err(PasswordError::TooLong);
        }
        Ok(Self(password))
    }

    pub fn as_str(&self) -> &str {
        &self.0.0
    }
}

pub enum PasswordError {
    TooShort,
    TooLong,
}

impl PasswordError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::TooShort | Self::TooLong => "Password must be 8–72 characters",
        }
    }
}
