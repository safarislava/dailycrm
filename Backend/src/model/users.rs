use crate::contract::Users;
use crate::model::password_hash::{PasswordHash, VerifyError};
use crate::model::user::User;
use crate::model::username::ValidUsername;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgUsers {
    pool: PgPool,
}

impl PgUsers {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Users for PgUsers {
    async fn user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: Uuid,
            password_hash: String,
        }
        let row =
            sqlx::query_as::<_, Row>("SELECT id, password_hash FROM users WHERE username = $1")
                .bind(username)
                .fetch_optional(&self.pool)
                .await?;

        Ok(row.map(|r| User::new(r.id, PasswordHash::new(r.password_hash))))
    }

    async fn username(&self, id: Uuid) -> Result<Option<String>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            username: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT username FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.username))
    }

    async fn update_username(
        &self,
        id: Uuid,
        username: &ValidUsername,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("UPDATE users SET username = $2 WHERE id = $1")
            .bind(id)
            .bind(username)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(sqlx::Error::Database(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn password_verification(&self, id: Uuid, password: &str) -> Result<(), VerifyError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            password_hash: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT password_hash FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| VerifyError::Internal)?;

        let hash = match row {
            Some(r) => PasswordHash::new(r.password_hash),
            None => return Err(VerifyError::Internal),
        };

        hash.verify(password).await?;
        Ok(())
    }

    async fn update_password(&self, id: Uuid, new_hash: &PasswordHash) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1")
            .bind(id)
            .bind(new_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
