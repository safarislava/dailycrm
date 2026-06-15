use crate::model::credential::contract::hash::{Hash, HashError};
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
    async fn value(&self) -> Result<String, HashError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            password_hash: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT password_hash FROM users WHERE id = $1")
            .bind(self.user.id())
            .fetch_one(self.pool.as_ref())
            .await.map_err(|e| HashError::Internal(Box::from(e)))?;
        Ok(row.password_hash)
    }
}
