#[async_trait::async_trait]
pub trait List: Send + Sync {
    type Output;
    async fn items(&self) -> Result<Vec<Self::Output>, sqlx::Error>;
}
