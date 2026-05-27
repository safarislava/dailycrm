use async_trait::async_trait;
use uuid::Uuid;

use crate::model::stage::{DetailedStage, Stage};

#[async_trait]
pub trait Stages: Send + Sync {
    async fn list(&self, project_id: Uuid) -> Result<Vec<Stage>, sqlx::Error>;

    async fn detailed_stage(
        &self,
        project_id: Uuid,
        position: i32,
    ) -> Result<DetailedStage, sqlx::Error>;

    async fn append(&self, project_id: Uuid, title: String) -> Result<(), sqlx::Error>;

    async fn insert(
        &self,
        project_id: Uuid,
        position: i32,
        title: String,
    ) -> Result<(), sqlx::Error>;

    async fn remove(&self, project_id: Uuid, position: i32) -> Result<(), sqlx::Error>;
}
