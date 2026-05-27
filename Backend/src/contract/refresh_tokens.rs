use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait RefreshTokens: Send + Sync {
    async fn user_id_with_jti_revocation(&self, jti: Uuid) -> Result<Option<Uuid>, sqlx::Error>;
    async fn revoke(&self, jti: Uuid) -> Result<(), sqlx::Error>;
}
