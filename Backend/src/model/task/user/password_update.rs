use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::credential::hash::Hash;
use crate::model::credential::hash_verification::VerificationError;
use crate::model::task::contract::task::Task;
use crate::model::user::contract::protected::Protected;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct PasswordUpdate {
    pool: Arc<PgPool>,
    protected_user: Box<dyn Protected<Output = User>>,
    new_password: Box<dyn Contentable<Output = Hash>>,
}

impl PasswordUpdate {
    pub fn new(
        pool: Arc<PgPool>,
        protected_user: Box<dyn Protected<Output = User>>,
        new_password: Box<dyn Contentable<Output = Hash>>,
    ) -> Self {
        Self {
            pool,
            protected_user,
            new_password,
        }
    }
}

#[async_trait::async_trait]
impl Task for PasswordUpdate {
    type Output = ();

    async fn done(&self) -> Result<(), BoxError> {
        let user = self.protected_user.unprotected().await?;
        let hash = self
            .new_password
            .content()
            .await
            .map_err(|_| VerificationError::Internal)?;
        sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1")
            .bind(user.id())
            .bind(
                hash.content()
                    .await
                    .map_err(|_| VerificationError::Internal)?,
            )
            .execute(self.pool.as_ref())
            .await
            .map_err(|_| VerificationError::Internal)?;
        Ok(())
    }
}
