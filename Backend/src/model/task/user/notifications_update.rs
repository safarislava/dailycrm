use crate::common::BoxError;
use crate::model::task::contract::task::Task;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotificationsUpdate {
    pool: Arc<PgPool>,
    user: User,
    enabled: bool,
}

impl NotificationsUpdate {
    pub fn new(pool: Arc<PgPool>, user: User, enabled: bool) -> Self {
        Self {
            pool,
            user,
            enabled,
        }
    }
}

#[async_trait::async_trait]
impl Task for NotificationsUpdate {
    type Output = ();

    async fn done(&self) -> Result<(), BoxError> {
        sqlx::query("UPDATE users SET notifications_enabled = $2 WHERE id = $1")
            .bind(self.user.id())
            .bind(self.enabled)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
