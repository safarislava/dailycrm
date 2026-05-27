use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait StageFields: Send + Sync {
    async fn update_title(
        &self,
        project_id: Uuid,
        position: i32,
        title: String,
    ) -> Result<(), sqlx::Error>;

    async fn update_deadline(
        &self,
        project_id: Uuid,
        position: i32,
        deadline: Option<DateTime<Utc>>,
    ) -> Result<(), sqlx::Error>;

    async fn update_description(
        &self,
        project_id: Uuid,
        position: i32,
        description: Option<String>,
    ) -> Result<(), sqlx::Error>;

    async fn update_cost(
        &self,
        project_id: Uuid,
        position: i32,
        cost: Option<i32>,
    ) -> Result<(), sqlx::Error>;

    async fn update_completed(
        &self,
        project_id: Uuid,
        position: i32,
        completed: bool,
    ) -> Result<(), sqlx::Error>;
}
