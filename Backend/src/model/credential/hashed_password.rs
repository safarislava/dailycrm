use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::credential::hash::Hash;
use crate::model::credential::valid_password::ValidPassword;

pub struct HashedPassword(ValidPassword);

impl HashedPassword {
    pub fn new(password: ValidPassword) -> Self {
        Self(password)
    }
}

#[async_trait::async_trait]
impl Contentable for HashedPassword {
    type Output = Hash;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        let raw = self.0.content().await?;
        actix_web::rt::task::spawn_blocking(move || {
            bcrypt::hash(&raw, bcrypt::DEFAULT_COST)
                .map(Hash::new)
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
