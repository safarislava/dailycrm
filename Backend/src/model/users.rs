use crate::model::user::{PasswordHash, ValidUsername};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct Users {
    pool: PgPool,
}

impl Users {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[allow(dead_code)]
    pub async fn register(&self, username: &ValidUsername, password_hash: &PasswordHash) -> Result<Uuid, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: Uuid,
        }
        let row: Row = sqlx::query_as(
            "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id",
        )
        .bind(username.as_str())
        .bind(password_hash.as_str())
        .fetch_one(&self.pool)
        .await?;
        Ok(row.id)
    }

    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<(Uuid, PasswordHash)>, sqlx::Error> {
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
        Ok(row.map(|r| (r.id, PasswordHash::new(r.password_hash))))
    }

    pub async fn username_by_id(&self, id: Uuid) -> Result<Option<String>, sqlx::Error> {
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

    pub async fn password_hash_by_id(&self, id: Uuid) -> Result<Option<PasswordHash>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            password_hash: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT password_hash FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| PasswordHash::new(r.password_hash)))
    }

    pub async fn update_username(&self, id: Uuid, username: &ValidUsername) -> Result<bool, sqlx::Error> {
        let rows = sqlx::query("UPDATE users SET username = $2 WHERE id = $1")
            .bind(id)
            .bind(username.as_str())
            .execute(&self.pool)
            .await;
        match rows {
            Ok(_) => Ok(true),
            Err(sqlx::Error::Database(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub async fn update_password(&self, id: Uuid, new_hash: &PasswordHash) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1")
            .bind(id)
            .bind(new_hash.as_str())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
