use async_trait::async_trait;
use uuid::Uuid;

use crate::model::password_hash::{PasswordHash, VerifyError};
use crate::model::user::User;
use crate::model::username::ValidUsername;

#[async_trait]
pub trait Users: Send + Sync {
    async fn user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error>;
    async fn username(&self, id: Uuid) -> Result<Option<String>, sqlx::Error>;
    async fn update_username(
        &self,
        id: Uuid,
        username: &ValidUsername,
    ) -> Result<bool, sqlx::Error>;
    async fn password_verification(&self, id: Uuid, password: &str) -> Result<(), VerifyError>;
    async fn update_password(&self, id: Uuid, new_hash: &PasswordHash) -> Result<(), sqlx::Error>;
}
