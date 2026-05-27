use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Attachment {
    id: Uuid,
    filename: String,
    mime_type: String,
    size_bytes: i64,
    created_at: DateTime<Utc>,
    download_url: String,
}

impl Attachment {
    pub fn new(
        id: Uuid,
        filename: String,
        mime_type: String,
        size_bytes: i64,
        created_at: DateTime<Utc>,
        download_url: String,
    ) -> Self {
        Self {
            id,
            filename,
            mime_type,
            size_bytes,
            created_at,
            download_url,
        }
    }
}
