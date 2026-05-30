use crate::common::BoxError;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::user::user::User;

#[async_trait::async_trait]
pub trait UsernameSearch {
    async fn found(&self, username: ValidUsername) -> Result<Option<User>, BoxError>;
}
