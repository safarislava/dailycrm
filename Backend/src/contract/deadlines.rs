use async_trait::async_trait;

use crate::model::stage::StageSummaryWithProjectTitle;

#[async_trait]
pub trait Deadlines: Send + Sync {
    async fn list(&self) -> Result<Vec<StageSummaryWithProjectTitle>, sqlx::Error>;
}
