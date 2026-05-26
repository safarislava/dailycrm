use crate::model::password_hash::{PasswordHash, ValidPasswordHash, VerifyError};
use crate::model::username::ValidUsername;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserLink {
    id: Uuid,
    pool: PgPool,
}

impl UserLink {
    pub fn new(id: Uuid, pool: PgPool) -> Self {
        Self { id, pool }
    }

    pub async fn username(&self) -> Result<Option<String>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            username: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT username FROM users WHERE id = $1")
            .bind(self.id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.username))
    }

    pub async fn password_verification(&self, password: &str) -> Result<(), VerifyError> {
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

        ValidPasswordHash::try_new(hash, password).await?;
        Ok(())
    }

    pub async fn update_username(&self, username: &ValidUsername) -> Result<bool, sqlx::Error> {
        let rows = sqlx::query("UPDATE users SET username = $2 WHERE id = $1")
            .bind(self.id)
            .bind(username.as_str())
            .execute(&self.pool)
            .await;
        match rows {
            Ok(_) => Ok(true),
            Err(sqlx::Error::Database(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub async fn update_password(&self, new_hash: &PasswordHash) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1")
            .bind(self.id)
            .bind(new_hash.as_str())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
