use async_trait::async_trait;
use uuid::Uuid;

use crate::model::invites::RegisterWithInviteResult;
use crate::model::password_hash::PasswordHash;
use crate::model::username::ValidUsername;

#[async_trait]
pub trait Invites: Send + Sync {
    async fn create(&self, created_by: Uuid) -> Result<Uuid, sqlx::Error>;

    async fn consume_and_register(
        &self,
        token: Uuid,
        username: &ValidUsername,
        password_hash: &PasswordHash,
    ) -> Result<RegisterWithInviteResult, sqlx::Error>;
}