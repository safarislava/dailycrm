use async_trait::async_trait;

#[async_trait]
pub trait Details: Send + Sync {
    type Detail;

    async fn details(&self) -> Result<Self::Detail, sqlx::Error>;
}