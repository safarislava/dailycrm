use crate::common::BoxError;
use crate::model::credential::contract::hash::Hash;
use crate::model::task::contract::task::Task;
use crate::model::user::contract::protected_user::ProtectedUser;
use sqlx::PgPool;
use std::sync::Arc;

pub struct PasswordUpdate {
    pool: Arc<PgPool>,
    protected_user: Box<dyn ProtectedUser>,
    new_password: Box<dyn Hash>,
}

impl PasswordUpdate {
    pub fn new(
        pool: Arc<PgPool>,
        protected_user: Box<dyn ProtectedUser>,
        new_password: Box<dyn Hash>,
    ) -> Self {
        Self { pool, protected_user, new_password }
    }
}

#[async_trait::async_trait]
impl Task for PasswordUpdate {
    type Output = ();

    async fn done(&self) -> Result<(), BoxError> {
        let user = self.protected_user.unprotected().await?;
        let hash = self.new_password.value().await?;
        sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1")
            .bind(user.id())
            .bind(hash)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
