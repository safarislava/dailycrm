use crate::model::credential::hash::Hash;
use crate::model::credential::hash_verification::{HashVerification, VerificationError};
use crate::model::credential::valid_password::ValidPassword;
use crate::model::user::contract::protected::Protected;
use crate::model::user::user::User;
use sqlx::PgPool;

pub struct ProtectedUser {
    pool: PgPool,
    user: User,
    password: ValidPassword,
}

impl ProtectedUser {
    pub fn new(pool: PgPool, user: User, password: ValidPassword) -> Self {
        Self {
            pool,
            user,
            password,
        }
    }
}

#[async_trait::async_trait]
impl Protected for ProtectedUser {
    type Output = User;

    async fn unprotected(&self) -> Result<Self::Output, VerificationError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            password_hash: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT password_hash FROM users WHERE id = $1")
            .bind(self.user.id())
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| VerificationError::Internal)?;

        let hash = match row {
            Some(r) => Hash::new(r.password_hash),
            None => return Err(VerificationError::Internal),
        };

        match HashVerification::new(hash, self.password.clone())
            .status()
            .await
        {
            Ok(_) => Ok(self.user.clone()),
            Err(e) => Err(e),
        }
    }
}
