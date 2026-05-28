use crate::contract::contentable::Contentable;
use crate::model::credential::hash::Hash;
use crate::model::credential::valid_password::ValidPassword;

pub struct HashVerification {
    hash: Hash,
    password: ValidPassword,
}

impl HashVerification {
    pub fn new(hash: Hash, password: ValidPassword) -> Self {
        Self { hash, password }
    }

    pub async fn status(&self) -> Result<(), VerificationError> {
        let hash = self
            .hash
            .content()
            .await
            .map_err(|_| VerificationError::Internal)?;
        let password = self
            .password
            .content()
            .await
            .map_err(|_| VerificationError::Internal)?;
        match actix_web::rt::task::spawn_blocking(move || bcrypt::verify(&password, &hash)).await {
            Ok(Ok(true)) => Ok(()),
            Ok(Ok(false)) => Err(VerificationError::WrongPassword),
            _ => Err(VerificationError::Internal),
        }
    }
}

#[derive(Debug)]
pub enum VerificationError {
    WrongPassword,
    Internal,
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            VerificationError::WrongPassword => "Wrong password",
            VerificationError::Internal => "Internal error",
        })
    }
}

impl std::error::Error for VerificationError {}
