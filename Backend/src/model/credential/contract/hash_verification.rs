#[async_trait::async_trait]
pub trait UserVerification: Send + Sync + 'static {
    async fn status(&self) -> Result<(), VerificationError>;
}

#[derive(Debug)]
pub enum VerificationError {
    Wrong,
    Internal,
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            VerificationError::Wrong => "Wrong password",
            VerificationError::Internal => "Internal error",
        })
    }
}

impl std::error::Error for VerificationError {}