use crate::model::authorized_user::ConfirmingUser;
use crate::model::password::ValidPassword;
use crate::model::username::ValidUsername;
use sqlx::PgPool;
use uuid::Uuid;

pub struct User {
    pool: PgPool,
    id: Uuid,
}

impl User {
    pub fn new(pool: PgPool, id: Uuid) -> Self {
        Self { pool, id }
    }

    pub fn confirming(&self, password: ValidPassword) -> ConfirmingUser {
        ConfirmingUser::new(self.pool.clone(), self.id, password)
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

    pub async fn update_username(
        &self,
        username: &ValidUsername,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("UPDATE users SET username = $2 WHERE id = $1")
            .bind(self.id)
            .bind(username)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(sqlx::Error::Database(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }
}