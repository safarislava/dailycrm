use async_trait::async_trait;

use crate::model::stage::StageWithProjectTitle;

#[async_trait]
pub trait Deadlines: Send + Sync {
    async fn list(&self) -> Result<Vec<StageWithProjectTitle>, sqlx::Error>;
}