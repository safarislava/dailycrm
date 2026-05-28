use crate::contract::Users;
use crate::model::user::User;
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
    fn user(&self, id: Uuid) -> User {
        User::new(self.pool.clone(), id)
    }

    async fn user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let id = sqlx::query_scalar::<_, Uuid>("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(id.map(|id| User::new(self.pool.clone(), id)))
    }
}
