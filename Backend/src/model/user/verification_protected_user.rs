use crate::model::credential::contract::hash_verification::{UserVerification, VerificationError};
use crate::model::user::contract::protected_user::ProtectedUser;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct VerificationProtectedUser {
    pool: Arc<PgPool>,
    user: User,
    verification: Box<dyn UserVerification>,
}

impl VerificationProtectedUser {
    pub fn new(pool: Arc<PgPool>, user: User, verification: impl UserVerification) -> Self {
        Self {
            pool,
            user,
            verification: Box::new(verification),
        }
    }
}

#[async_trait::async_trait]
impl ProtectedUser for VerificationProtectedUser {
    async fn unprotected(&self) -> Result<User, VerificationError> {
        match self.verification.status().await {
            Ok(_) => Ok(self.user.clone()),
            Err(e) => Err(e),
        }
    }
}
