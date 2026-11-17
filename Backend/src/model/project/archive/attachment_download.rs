use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::attachment::id::AttachmentId;
use crate::storage::{FileStream, Storage};
use std::sync::Arc;

pub struct ArchivedAttachmentDownload {
    storage: Arc<Storage>,
    project_id: ProjectId,
    attachment_id: AttachmentId,
}

impl ArchivedAttachmentDownload {
    pub fn new(storage: Arc<Storage>, project_id: ProjectId, attachment_id: AttachmentId) -> Self {
        Self {
            storage,
            project_id,
            attachment_id,
        }
    }
}

#[async_trait::async_trait]
impl Task for ArchivedAttachmentDownload {
    type Output = (FileStream, i64, String, String);

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let storage_key = format!("archives/{}.json", self.project_id.id());
        let bytes = self.storage.read_bytes(&storage_key).await?;
        let payload: serde_json::Value = serde_json::from_slice(&bytes)?;

        let attachment_id_str = self.attachment_id.id().to_string();
        let attachment = payload["attachments"]
            .as_array()
            .and_then(|arr| {
                arr.iter()
                    .find(|a| a["id"].as_str() == Some(&attachment_id_str))
            })
            .ok_or("Attachment not found in archive")?;

        let filename = attachment["filename"]
            .as_str()
            .unwrap_or("file")
            .to_string();
        let mime_type = attachment["mime_type"]
            .as_str()
            .unwrap_or("application/octet-stream")
            .to_string();
        let size_bytes = attachment["size_bytes"].as_i64().unwrap_or(0);

        let stream = self.storage.stream(&attachment_id_str).await?;

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
