use async_trait::async_trait;
use uuid::Uuid;

use crate::model::attachment::Attachment;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[async_trait]
pub trait Attachments: Send + Sync {
    async fn list(
        &self,
        project_id: Uuid,
        stage_position: i32,
    ) -> Result<Vec<Attachment>, sqlx::Error>;

    async fn upload(
        &self,
        project_id: Uuid,
        stage_position: i32,
        filename: String,
        mime_type: String,
        data: Vec<u8>,
    ) -> Result<Uuid, BoxError>;

    async fn download(
        &self,
        project_id: Uuid,
        stage_position: i32,
        id: Uuid,
    ) -> Result<(Vec<u8>, String, String), BoxError>;

    async fn delete(&self, project_id: Uuid, id: Uuid) -> Result<(), BoxError>;
}
