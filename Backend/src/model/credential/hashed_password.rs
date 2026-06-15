use crate::model::credential::contract::hash::{Hash, HashError};
use crate::model::credential::contract::password::Password;

pub struct HashedPassword(Box<dyn Password>);

impl HashedPassword {
    pub fn new(password: impl Password) -> Self {
        Self(Box::new(password))
    }
}

#[async_trait::async_trait]
impl Hash for HashedPassword {
    async fn value(&self) -> Result<String, HashError> {
        let raw = self.0.value().map_err(|e| HashError::Internal(Box::new(e)))?;
        actix_web::rt::task::spawn_blocking(move || {
            bcrypt::hash(&raw, bcrypt::DEFAULT_COST)
                .map_err(|_| HashError::Bcrypt)
        })
        .await
        .map_err(|_| HashError::Task)?
    }
}
