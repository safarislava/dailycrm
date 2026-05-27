use crate::model::access_token::AccessToken;
use crate::model::password_hash::PasswordHash;
use crate::model::refresh_token::RefreshToken;
use uuid::Uuid;

pub enum LoginError {
    WrongPassword,
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
        self.password_hash
            .verify(password)
            .await
            .map_err(|_| LoginError::WrongPassword)?;

        Ok((AccessToken::new(self.id), RefreshToken::new(self.id)))
    }
}
