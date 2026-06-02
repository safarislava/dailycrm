use crate::common::BoxError;
use crate::model::project::attachment::Attachment;
use crate::model::task::contract::task::Task;
use crate::storage::{FileStream, Storage};
use sqlx::PgPool;
use std::sync::Arc;

pub struct AttachmentDownload {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    attachment: Attachment,
}

impl AttachmentDownload {
    pub fn new(pool: Arc<PgPool>, storage: Arc<Storage>, attachment: Attachment) -> Self {
        Self {
            pool,
            storage,
            attachment,
        }
    }
}

#[async_trait::async_trait]
impl Task for AttachmentDownload {
    type Output = (FileStream, i64, String, String);

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let (filename, mime_type, size_bytes): (String, String, i64) =
            sqlx::query_as("SELECT filename, mime_type, size_bytes FROM attachments WHERE id = $1")
                .bind(self.attachment.id())
                .fetch_one(self.pool.as_ref())
                .await?;
        let stream = self
            .storage
            .stream(&self.attachment.id().to_string())
            .await?;
        let encoded: String = filename
            .bytes()
            .flat_map(|b| {
                if b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_' | b'~') {
                    vec![b as char]
                } else {
                    format!("%{:02X}", b).chars().collect::<Vec<_>>()
                }
            })
            .collect();
        let ascii_fallback = filename.replace('"', "\\\"");
        let disposition = format!(
            "attachment; filename=\"{}\"; filename*=UTF-8''{}",
            ascii_fallback, encoded
        );
        Ok((stream, size_bytes, mime_type, disposition))
    }
}
