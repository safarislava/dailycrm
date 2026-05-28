use crate::common::BoxError;
use crate::contract::sting_contentable::StringContentable;
use crate::model::hash::Hash;
use crate::model::valid_password::ValidPassword;

pub struct HashedPassword(ValidPassword);

impl HashedPassword {
    pub fn new(password: ValidPassword) -> Self {
        Self(password)
    }
}

impl HashedPassword {
    pub(crate) async fn content(&self) -> Result<Hash, BoxError> {
        let raw = self.0.content().await?;
        actix_web::rt::task::spawn_blocking(move || {
            bcrypt::hash(&raw, bcrypt::DEFAULT_COST)
                .map(|hash| Hash::new(hash))
                .map_err(|e| Box::new(e) as BoxError)
        })
        .await
        .map_err(|e| Box::new(e) as BoxError)?
    }
}