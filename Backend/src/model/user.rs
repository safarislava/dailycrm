use crate::model::access_token::AccessToken;
use crate::model::password_hash::{PasswordHash, ValidPasswordHash};
use crate::model::refresh_token::RefreshToken;
use uuid::Uuid;

pub enum LoginError {
    WrongPassword,
    Internal,
}

pub struct User {
    id: Uuid,
    password_hash: PasswordHash,
}

impl User {
    pub fn new(id: Uuid, password_hash: PasswordHash) -> Self {
        Self { id, password_hash }
    }

    pub async fn tokens(&self, password: &str) -> Result<(AccessToken, RefreshToken), LoginError> {
        ValidPasswordHash::try_new(self.password_hash.clone(), password)
            .await
            .map_err(|_| LoginError::WrongPassword)?;

        let access_token = AccessToken::new(self.id).map_err(|_| LoginError::Internal)?;
        let refresh_token = RefreshToken::new(self.id).map_err(|_| LoginError::Internal)?;

        Ok((access_token, refresh_token))
    }
}