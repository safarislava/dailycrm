use crate::contract::RefreshTokens;
use crate::contract::contentable::Contentable;
use crate::model::session::access_token::AccessToken;
use crate::model::credential::hash::Hash;
use crate::model::credential::hash_verification::{HashVerification, VerificationError};
use crate::model::credential::hashed_password::HashedPassword;
use crate::model::session::refresh_token::NewRefreshToken;
use crate::model::credential::valid_password::ValidPassword;
use sqlx::PgPool;
use uuid::Uuid;

pub struct ConfirmedUser {
    pool: PgPool,
    id: Uuid,
    password: ValidPassword,
}

impl ConfirmedUser {
    pub fn new(pool: PgPool, id: Uuid, password: ValidPassword) -> Self {
        Self { pool, id, password }
    }

    pub async fn tokens(
        &self,
        refresh_tokens: &dyn RefreshTokens,
    ) -> Result<(AccessToken, NewRefreshToken), VerificationError> {
        self.verification().await?;

        let access_token = AccessToken::new(self.id);
        let refresh_token = refresh_tokens
            .new_token(self.id)
            .await
            .map_err(|_| VerificationError::Internal)?;
        Ok((access_token, refresh_token))
    }

    pub async fn update_password(
        &self,
        new_password: ValidPassword,
    ) -> Result<(), VerificationError> {
        self.verification().await?;

        let hashed_password = HashedPassword::new(new_password);
        let hash = hashed_password
            .content()
            .await
            .map_err(|_| VerificationError::Internal)?;
        sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1")
            .bind(self.id)
            .bind(
                hash.content()
                    .await
                    .map_err(|_| VerificationError::Internal)?,
            )
            .execute(&self.pool)
            .await
            .map_err(|_| VerificationError::Internal)?;
        Ok(())
    }

    async fn verification(&self) -> Result<(), VerificationError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            password_hash: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT password_hash FROM users WHERE id = $1")
            .bind(self.id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| VerificationError::Internal)?;

        let hash = match row {
            Some(r) => Hash::new(r.password_hash),
            None => return Err(VerificationError::Internal),
        };

        HashVerification::new(hash, self.password.clone())
            .status()
            .await
    }
}
