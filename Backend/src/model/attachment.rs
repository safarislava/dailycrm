use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::storage::Storage;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct Attachment {
    pool: PgPool,
    storage: Storage,
    project_id: Uuid,
    stage_position: i32,
    id: Uuid,
}

impl Attachment {
    pub fn new(
        pool: PgPool,
        storage: Storage,
        project_id: Uuid,
        stage_position: i32,
        id: Uuid,
    ) -> Self {
        Attachment {
            pool,
            storage,
            project_id,
            stage_position,
            id,
        }
    }

    pub async fn download(&self) -> Result<(Vec<u8>, String, String), BoxError> {
        let (filename, mime_type): (String, String) = sqlx::query_as(
            "SELECT filename, mime_type FROM attachments
             WHERE id = $1 AND project_id = $2 AND stage_position = $3",
        )
        .bind(self.id)
        .bind(self.project_id)
        .bind(self.stage_position)
        .fetch_one(&self.pool)
        .await?;

        let data = self.storage.get_bytes(&self.id.to_string()).await?;

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

        Ok((data, mime_type, disposition))
    }

    pub async fn delete(&self) -> Result<(), BoxError> {
        let _ = self.storage.delete(&self.id.to_string()).await;

        let result = sqlx::query("DELETE FROM attachments WHERE id = $1 AND project_id = $2")
            .bind(self.id)
            .bind(self.project_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err("attachment not found".into());
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct AttachmentSummary {
    id: Uuid,
    filename: String,
    mime_type: String,
    size_bytes: i64,
    created_at: DateTime<Utc>,
    download_url: String,
}

impl AttachmentSummary {
    pub fn new(
        id: Uuid,
        filename: String,
        mime_type: String,
        size_bytes: i64,
        created_at: DateTime<Utc>,
        download_url: String,
    ) -> Self {
        AttachmentSummary {
            id,
            filename,
            mime_type,
            size_bytes,
            created_at,
            download_url,
        }
    }
}