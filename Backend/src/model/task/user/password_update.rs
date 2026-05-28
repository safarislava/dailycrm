use crate::common::BoxError;
use crate::contract::contentable::Contentable;
use crate::contract::task::Task;
use crate::model::credential::hash_verification::VerificationError;
use crate::model::credential::hashed_password::HashedPassword;
use crate::model::credential::valid_password::ValidPassword;
use crate::model::user::protected_user::ProtectedUser;
use sqlx::PgPool;

pub struct PasswordUpdate {
    pool: PgPool,
    protected_user: ProtectedUser,
    new_password: ValidPassword,
}

impl PasswordUpdate {
    pub fn new(pool: PgPool, protected_user: ProtectedUser, new_password: ValidPassword) -> Self {
        Self {
            pool,
            protected_user,
            new_password,
        }
    }
}

impl Task for PasswordUpdate {
    type Output = ();

    async fn output(&self) -> Result<(), BoxError> {
        let user = self.protected_user.unprotected().await?;

        let hashed_password = HashedPassword::new(self.new_password.clone());
        let hash = hashed_password
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
            .execute(&self.pool)
            .await
            .map_err(|_| VerificationError::Internal)?;
        Ok(())
    }
}
