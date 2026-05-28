use crate::common::BoxError;
use crate::model::credential::hash::Hash;
use crate::model::user::invites::RegisterWithInviteResult;
use crate::model::credential::valid_username::ValidUsername;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Invites: Send + Sync {
    async fn create(&self, created_by: Uuid) -> Result<Uuid, sqlx::Error>;

    async fn consume_and_register(
        &self,
        token: Uuid,
        username: &ValidUsername,
        password_hash: &Hash,
    ) -> Result<RegisterWithInviteResult, BoxError>;
}
