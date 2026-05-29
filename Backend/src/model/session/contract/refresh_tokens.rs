use async_trait::async_trait;
use uuid::Uuid;

use crate::model::session::refresh_token::{NewRefreshToken, RefreshToken};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[async_trait]
pub trait RefreshTokens: Send + Sync {
    fn token(&self, jti: Uuid) -> RefreshToken;
    async fn new_token(&self, user_id: Uuid) -> Result<NewRefreshToken, BoxError>;
}
