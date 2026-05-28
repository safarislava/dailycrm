use async_trait::async_trait;

use crate::model::stage::{Stage, StageSummary};

#[async_trait]
pub trait Stages: Send + Sync {
    fn stage(&self, position: i32) -> Stage;

    async fn list(&self) -> Result<Vec<StageSummary>, sqlx::Error>;

    async fn append(&self, title: String) -> Result<(), sqlx::Error>;

    async fn insert(&self, position: i32, title: String) -> Result<(), sqlx::Error>;
}
