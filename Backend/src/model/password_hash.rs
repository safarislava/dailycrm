use crate::model::password::ValidPassword;

#[derive(Clone)]
pub struct PasswordHash(String);

pub enum HashError {
    Bcrypt,
    Task,
}

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        PasswordHash(hash)
    }

    pub async fn new_from_password(password: ValidPassword) -> Result<Self, HashError> {
        match actix_web::rt::task::spawn_blocking(move || {
            bcrypt::hash(password.as_str(), bcrypt::DEFAULT_COST)
        })
        .await
        {
            Ok(Ok(hash)) => Ok(Self(hash)),
            Ok(Err(_)) => Err(HashError::Bcrypt),
            Err(_) => Err(HashError::Task),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[allow(dead_code)]
pub struct ValidPasswordHash(PasswordHash);

pub enum VerifyError {
    WrongPassword,
    Internal,
}

impl ValidPasswordHash {
    pub async fn try_new(hash: PasswordHash, password: &str) -> Result<Self, VerifyError> {
        let raw = hash.0.clone();
        let password = password.to_owned();
        match actix_web::rt::task::spawn_blocking(move || bcrypt::verify(&password, &raw)).await {
            Ok(Ok(true)) => Ok(Self(hash)),
            Ok(Ok(false)) => Err(VerifyError::WrongPassword),
            _ => Err(VerifyError::Internal),
        }
    }
}
