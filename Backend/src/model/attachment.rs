use crate::storage::Storage;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize)]
pub struct Attachment {
    id: Uuid,
    filename: String,
    mime_type: String,
    size_bytes: i64,
    created_at: DateTime<Utc>,
    download_url: String,
    #[serde(skip)]
    storage: Storage,
}

impl Attachment {
    pub fn new(
        id: Uuid,
        filename: String,
        mime_type: String,
        size_bytes: i64,
        created_at: DateTime<Utc>,
        download_url: String,
        storage: Storage,
    ) -> Self {
        Self {
            id,
            filename,
            mime_type,
            size_bytes,
            created_at,
            download_url,
            storage,
        }
    }

    pub async fn content(&self) -> Result<(Vec<u8>, String, String), BoxError> {
        let data = self.storage.get_bytes(&self.id.to_string()).await?;
        let encoded: String = self
            .filename
            .bytes()
            .flat_map(|b| {
                if b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_' | b'~') {
                    vec![b as char]
                } else {
                    format!("%{:02X}", b).chars().collect::<Vec<_>>()
                }
            })
            .collect();
        let ascii_fallback = self.filename.replace('"', "\\\"");
        let disposition = format!(
            "attachment; filename=\"{}\"; filename*=UTF-8''{}",
            ascii_fallback, encoded
        );
        Ok((data, self.mime_type.clone(), disposition))
    }
}
