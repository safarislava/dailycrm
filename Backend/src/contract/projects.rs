use async_trait::async_trait;
use uuid::Uuid;

use crate::model::project::Project;

#[async_trait]
pub trait Projects: Send + Sync {
    async fn list(&self) -> Result<Vec<Project>, sqlx::Error>;
    async fn register(&self, title: &str) -> Result<(), sqlx::Error>;
    async fn rename(&self, id: Uuid, title: &str) -> Result<(), sqlx::Error>;
    async fn remove(&self, id: Uuid) -> Result<(), sqlx::Error>;
}