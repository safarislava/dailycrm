use crate::common::BoxError;
use crate::model::credential::contract::hash::Hash;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct DbHash {
    pool: Arc<PgPool>,
    user: User,
}

impl DbHash {
    pub fn new(pool: Arc<PgPool>, user: User) -> Self {
        Self { pool, user }
    }
}

#[async_trait::async_trait]
impl Hash for DbHash {
    async fn value(&self) -> Result<String, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            password_hash: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT password_hash FROM users WHERE id = $1")
            .bind(self.user.id())
            .fetch_one(self.pool.as_ref())
            .await?;
        Ok(row.password_hash)
    }
}
