use crate::model::password_hash::{HashError, PasswordHash};

pub struct Password(pub String);

pub struct ValidPassword(Password);

impl Password {
    pub fn validated(self) -> Result<ValidPassword, PasswordError> {
        let len = self.0.len();
        if len < 8 {
            return Err(PasswordError::TooShort);
        }
        if len > 72 {
            return Err(PasswordError::TooLong);
        }
        Ok(ValidPassword(self))
    }
}

impl ValidPassword {
    pub async fn hashed(self) -> Result<PasswordHash, HashError> {
        let raw = self.0.0;
        match actix_web::rt::task::spawn_blocking(move || bcrypt::hash(&raw, bcrypt::DEFAULT_COST))
            .await
        {
            Ok(Ok(hash)) => Ok(PasswordHash::new(hash)),
            Ok(Err(_)) => Err(HashError::Bcrypt),
            Err(_) => Err(HashError::Task),
        }
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
