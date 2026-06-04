use crate::model::credential::contract::hash_verification::VerificationError;
use crate::model::user::user::User;

#[async_trait::async_trait]
pub trait ProtectedUser: Send + Sync {
    async fn unprotected(&self) -> Result<User, VerificationError>;
}
