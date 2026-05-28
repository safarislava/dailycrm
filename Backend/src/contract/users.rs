use async_trait::async_trait;
use uuid::Uuid;

use crate::model::user::user::User;

#[async_trait]
pub trait Users: Send + Sync {
    fn user(&self, id: Uuid) -> User;
    async fn user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error>;
}
