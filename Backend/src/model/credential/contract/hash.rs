use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Hash: Send + Sync + 'static {
    async fn value(&self) -> Result<String, HashError>;
}

pub enum HashError {
    Bcrypt,
    Task,
    Internal(BoxError),
}

impl std::fmt::Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bcrypt => f.write_str("Failed to hash password"),
            Self::Task => f.write_str("Blocking task failed"),
            Self::Internal(error) => std::fmt::Display::fmt(error, f),
        }
    }
}

impl std::fmt::Debug for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for HashError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Internal(error) => Some(error.as_ref()),
            _ => None,
        }
    }
}
