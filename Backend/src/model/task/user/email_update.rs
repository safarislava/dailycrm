use crate::common::BoxError;
use crate::model::task::contract::task::Task;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct EmailUpdate {
    pool: Arc<PgPool>,
    user: User,
    email: String,
}

impl EmailUpdate {
    pub fn new(pool: Arc<PgPool>, user: User, email: String) -> Self {
        Self { pool, user, email }
    }
}

#[async_trait::async_trait]
impl Task for EmailUpdate {
    type Output = ();

    async fn done(&self) -> Result<(), BoxError> {
        sqlx::query("UPDATE users SET email = $2 WHERE id = $1")
            .bind(self.user.id())
            .bind(&self.email)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
