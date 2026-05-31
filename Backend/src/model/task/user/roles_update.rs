use crate::common::BoxError;
use crate::model::task::contract::task::Task;
use crate::model::user::role::Role;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct RolesUpdate {
    pool: Arc<PgPool>,
    user: User,
    roles: Vec<Role>,
}

impl RolesUpdate {
    pub fn new(pool: Arc<PgPool>, user: User, roles: Vec<Role>) -> Self {
        Self { pool, user, roles }
    }
}

#[async_trait::async_trait]
impl Task for RolesUpdate {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM user_roles WHERE user_id = $1")
            .bind(self.user.id())
            .execute(&mut *tx)
            .await?;
        for role in &self.roles {
            sqlx::query("INSERT INTO user_roles (user_id, role) VALUES ($1, $2)")
                .bind(self.user.id())
                .bind(role)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}