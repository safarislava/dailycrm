use async_trait::async_trait;
use uuid::Uuid;

use crate::model::project::project::{Project, ProjectDetails};

#[async_trait]
pub trait Projects: Send + Sync {
    fn project(&self, id: Uuid) -> Project;
    async fn list(&self) -> Result<Vec<ProjectDetails>, sqlx::Error>;
    async fn register(&self, title: &str) -> Result<(), sqlx::Error>;
}
