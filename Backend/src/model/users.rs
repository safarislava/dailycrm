use crate::model::password_hash::PasswordHash;
use crate::model::user::User;
use crate::model::user_link::UserLink;
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

    pub fn user_link(&self, id: Uuid) -> UserLink {
        UserLink::new(id, self.pool.clone())
    }

    pub async fn user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
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
}
