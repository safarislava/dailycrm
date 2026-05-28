use crate::model::access_token::AccessToken;
use crate::model::password::ValidPassword;
use crate::model::password_hash::{PasswordHash, VerifyError};
use crate::model::refresh_token::RefreshToken;
use sqlx::PgPool;
use uuid::Uuid;

pub enum LoginError {
    WrongPassword,
}

pub enum UpdatePasswordError {
    WrongPassword,
    Internal,
}

pub struct ConfirmingUser {
    pool: PgPool,
    id: Uuid,
    password: ValidPassword,
}

impl ConfirmingUser {
    pub fn new(pool: PgPool, id: Uuid, password: ValidPassword) -> Self {
        Self { pool, id, password }
    }

    pub async fn tokens(&self) -> Result<(AccessToken, RefreshToken), LoginError> {
        self.verify().await.map_err(|_| LoginError::WrongPassword)?;
        Ok((AccessToken::new(self.id), RefreshToken::new(self.id)))
    }

    pub async fn update_password(
        &self,
        new_password: ValidPassword,
    ) -> Result<(), UpdatePasswordError> {
        self.verify().await.map_err(|e| match e {
            VerifyError::WrongPassword => UpdatePasswordError::WrongPassword,
            VerifyError::Internal => UpdatePasswordError::Internal,
        })?;

        let hash = new_password
            .hashed()
            .await
            .map_err(|_| UpdatePasswordError::Internal)?;

        sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1")
            .bind(self.id)
            .bind(&hash)
            .execute(&self.pool)
            .await
            .map_err(|_| UpdatePasswordError::Internal)?;
        Ok(())
    }

    async fn verify(&self) -> Result<(), VerifyError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            password_hash: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT password_hash FROM users WHERE id = $1")
            .bind(self.id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| VerifyError::Internal)?;

        let hash = match row {
            Some(r) => PasswordHash::new(r.password_hash),
            None => return Err(VerifyError::Internal),
        };

        self.password.matches(&hash).await
    }
}