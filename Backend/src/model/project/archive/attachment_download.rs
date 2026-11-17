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

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;
    use opendal::Operator;
    use opendal::services::Memory;
    use serde_json::json;
    use uuid::Uuid;

    #[actix_web::test]
    async fn downloads_file_from_archive() {
        let op = Operator::new(Memory::default()).unwrap();
        let storage = Arc::new(Storage::new(op));
        let project_id = ProjectId::new(Uuid::new_v4());
        let file_id = Uuid::new_v4();

        let payload = json!({
            "attachments": [
                {
                    "id": file_id.to_string(),
                    "parent_position": 0,
                    "stage_position": 1,
                    "filename": "act_01.pdf",
                    "mime_type": "application/pdf",
                    "size_bytes": 13,
                    "is_act": true
                }
            ]
        });

        let json_key = format!("archives/{}.json", project_id.id());
        storage
            .write_bytes(
                &json_key,
                serde_json::to_vec(&payload).unwrap(),
                "application/json",
            )
            .await
            .unwrap();

        let file_bytes = b"hello act pdf";
        storage
            .write_bytes(&file_id.to_string(), file_bytes.to_vec(), "application/pdf")
            .await
            .unwrap();

        let task = ArchivedAttachmentDownload::new(storage, project_id, AttachmentId::new(file_id));
        let (mut stream, size, mime, disposition) = task.perform().await.unwrap();

        assert_eq!(size, 13);
        assert_eq!(mime, "application/pdf");
        assert!(disposition.contains("act_01.pdf"));

        let mut read_bytes = Vec::new();
        while let Some(chunk) = stream.next().await {
            read_bytes.extend_from_slice(&chunk.unwrap());
        }
        assert_eq!(read_bytes, file_bytes);
    }
}
