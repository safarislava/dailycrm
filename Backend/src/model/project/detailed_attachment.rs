use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::attachment::Attachment;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct DetailedAttachment {
    pool: Arc<PgPool>,
    attachment: Attachment,
}

impl DetailedAttachment {
    pub fn new(pool: Arc<PgPool>, attachment: Attachment) -> Self {
        Self { pool, attachment }
    }
}

#[async_trait::async_trait]
impl Contentable for DetailedAttachment {
    type Output = serde_json::Value;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: Uuid,
            project_id: Uuid,
            stage_position: i32,
            filename: String,
            mime_type: String,
            size_bytes: i64,
            created_at: DateTime<Utc>,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT id, project_id, stage_position, filename, mime_type, size_bytes, created_at
             FROM attachments WHERE id = $1",
        )
        .bind(self.attachment.id())
        .fetch_one(self.pool.as_ref())
        .await?;
        let download_url = format!(
            "/api/projects/{}/stages/{}/attachments/{}/download",
            row.project_id, row.stage_position, row.id
        );
        Ok(serde_json::json!({
            "id": row.id,
            "filename": row.filename,
            "mime_type": row.mime_type,
            "size_bytes": row.size_bytes,
            "created_at": row.created_at,
            "download_url": download_url,
        }))
    }
}
