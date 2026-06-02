use crate::common::BoxError;
use crate::model::credential::contract::hash::Hash;
use crate::model::credential::contract::password::Password;

pub struct HashedPassword(Box<dyn Password>);

impl HashedPassword {
    pub fn new(password: impl Password) -> Self {
        Self(Box::new(password))
    }
}

#[async_trait::async_trait]
impl Hash for HashedPassword {
    async fn value(&self) -> Result<String, BoxError> {
        let raw = self.0.value()?;
        actix_web::rt::task::spawn_blocking(move || {
            bcrypt::hash(&raw, bcrypt::DEFAULT_COST)
                .map_err(|_| Box::new(HashError::Bcrypt) as BoxError)
        })
        .await
        .map_err(|_| Box::new(HashError::Task) as BoxError)?
    }
}

pub enum HashError {
    Bcrypt,
    Task,
}

impl std::fmt::Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Bcrypt => "Failed to hash password",
            Self::Task => "Blocking task failed",
        })
    }
}

impl std::fmt::Debug for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for HashError {}
