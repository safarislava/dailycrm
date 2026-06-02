use crate::common::BoxError;
use crate::model::credential::contract::username::Username;
use crate::model::user::user::User;

#[async_trait::async_trait]
pub trait UsernameSearch {
    async fn found(&self, username: impl Username) -> Result<Option<User>, BoxError>;
}
