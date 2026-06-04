use crate::model::project::contract::list::List;
use crate::model::project::stage::Stage;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AttachmentSummaries {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl AttachmentSummaries {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait]
impl List for AttachmentSummaries {
    type Output = serde_json::Value;

    async fn items(&self) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: Uuid,
            project_id: Uuid,
            parent_position: i32,
            stage_position: i32,
            filename: String,
            mime_type: String,
            size_bytes: i64,
            created_at: DateTime<Utc>,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT id, project_id, parent_position, stage_position, filename, mime_type, size_bytes, created_at \
             FROM attachments \
             WHERE project_id = $1 AND parent_position = $2 AND stage_position = $3 AND is_act = FALSE \
             ORDER BY created_at",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows.into_iter().map(|row| {
            let download_url = if row.parent_position == 0 {
                format!("/api/projects/{}/stages/{}/attachments/{}/download", row.project_id, row.stage_position, row.id)
            } else {
                format!("/api/projects/{}/stages/{}/sub/{}/attachments/{}/download", row.project_id, row.parent_position, row.stage_position, row.id)
            };
            serde_json::json!({
                "id": row.id,
                "filename": row.filename,
                "mime_type": row.mime_type,
                "size_bytes": row.size_bytes,
                "created_at": row.created_at,
                "download_url": download_url,
            })
        }).collect())
    }
}