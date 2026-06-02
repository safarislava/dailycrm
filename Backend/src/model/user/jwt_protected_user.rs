use crate::model::credential::contract::hash_verification::VerificationError;
use crate::model::session::refresh_token::RefreshToken;
use crate::model::task::contract::task::Task;
use crate::model::task::session::user_id_receipt::UserIdReceipt;
use crate::model::user::contract::protected_user::ProtectedUser;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct JwtProtectedUser {
    pool: Arc<PgPool>,
    refresh_token: RefreshToken,
}

impl JwtProtectedUser {
    pub fn new(pool: Arc<PgPool>, refresh_token: RefreshToken) -> Self {
        Self {
            pool,
            refresh_token,
        }
    }
}

#[async_trait::async_trait]
impl ProtectedUser for JwtProtectedUser {
    async fn unprotected(&self) -> Result<User, VerificationError> {
        match UserIdReceipt::new(self.pool.clone(), self.refresh_token.id())
            .done()
            .await
        {
            Ok(Some(id)) => Ok(User::new(id)),
            Ok(None) => Err(VerificationError::Wrong),
            Err(_) => Err(VerificationError::Internal),
        }
    }
}
