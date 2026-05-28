use async_trait::async_trait;
use uuid::Uuid;

use crate::model::attachment::{Attachment, AttachmentSummary};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[async_trait]
pub trait Attachments: Send + Sync {
    fn attachment(&self, id: Uuid) -> Attachment;

    async fn list(&self) -> Result<Vec<AttachmentSummary>, sqlx::Error>;

    async fn upload(
        &self,
        filename: String,
        mime_type: String,
        data: Vec<u8>,
    ) -> Result<Uuid, BoxError>;
}
