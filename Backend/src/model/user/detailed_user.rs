use crate::model::credential::username::Username;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct DetailedUser {
    pool: Arc<PgPool>,
    user: User,
}

impl DetailedUser {
    pub fn new(pool: Arc<PgPool>, user: User) -> DetailedUser {
        DetailedUser { pool, user }
    }

    pub async fn username(&self) -> Result<Option<Username>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            username: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT username FROM users WHERE id = $1")
            .bind(self.user.id())
            .fetch_optional(self.pool.as_ref())
            .await?;
        Ok(row.map(|r| Username::new(r.username)))
    }
}
