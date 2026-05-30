use crate::model::credential::hash_verification::VerificationError;

#[async_trait::async_trait]
pub trait Protected: Send + Sync {
    type Output;

    async fn unprotected(&self) -> Result<Self::Output, VerificationError>;
}
