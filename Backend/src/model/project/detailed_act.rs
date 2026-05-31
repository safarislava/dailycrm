use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::act::Act;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct DetailedAct {
    pool: Arc<PgPool>,
    act: Act,
}

impl DetailedAct {
    pub fn new(pool: Arc<PgPool>, act: Act) -> Self {
        Self { pool, act }
    }
}

#[async_trait::async_trait]
impl Contentable for DetailedAct {
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
        .bind(self.act.id())
        .fetch_one(self.pool.as_ref())
        .await?;
        let download_url = format!(
            "/api/projects/{}/stages/{}/act/{}/download",
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