use crate::model::credential::contract::hash::Hash;
use crate::model::credential::contract::hash_verification::{UserVerification, VerificationError};
use crate::model::credential::contract::password::Password;

pub struct HashUserVerification {
    hash: Box<dyn Hash>,
    password: Box<dyn Password>,
}

impl HashUserVerification {
    pub fn new(hash: impl Hash, password: impl Password) -> Self {
        Self { hash: Box::new(hash), password: Box::new(password) }
    }
}

#[async_trait::async_trait]
impl UserVerification for HashUserVerification {
    async fn status(&self) -> Result<(), VerificationError> {
        let hash = self.hash.value().await.map_err(|_| VerificationError::Internal)?;
        let password = self.password.value().map_err(|_| VerificationError::Wrong)?;
        match actix_web::rt::task::spawn_blocking(move || bcrypt::verify(&password, &hash)).await {
            Ok(Ok(true)) => Ok(()),
            Ok(Ok(false)) => Err(VerificationError::Wrong),
            _ => Err(VerificationError::Internal),
        }
    }
}