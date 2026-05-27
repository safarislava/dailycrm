use crate::contract::Attachments;
use crate::model::attachment::{Attachment, AttachmentSummary};
use crate::storage::Storage;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(sqlx::FromRow)]
struct AttachmentRow {
    id: Uuid,
    filename: String,
    mime_type: String,
    size_bytes: i64,
    created_at: DateTime<Utc>,
}

pub struct PgAttachments {
    pool: PgPool,
    storage: Storage,
    project_id: Uuid,
    stage_position: i32,
}

impl PgAttachments {
    pub fn new(pool: PgPool, storage: Storage, project_id: Uuid, stage_position: i32) -> Self {
        Self {
            pool,
            storage,
            project_id,
            stage_position,
        }
    }

    fn summary_from_row(&self, row: AttachmentRow) -> AttachmentSummary {
        let url = format!(
            "/api/projects/{}/stages/{}/attachments/{}/download",
            self.project_id, self.stage_position, row.id
        );
        AttachmentSummary::new(
            row.id,
            row.filename,
            row.mime_type,
            row.size_bytes,
            row.created_at,
            url,
        )
    }
}

#[async_trait]
impl Attachments for PgAttachments {
    fn attachment(&self, id: Uuid) -> Attachment {
        Attachment::new(
            self.pool.clone(),
            self.storage.clone(),
            self.project_id,
            self.stage_position,
            id,
        )
    }

    async fn list(&self) -> Result<Vec<AttachmentSummary>, sqlx::Error> {
        let rows = sqlx::query_as::<_, AttachmentRow>(
            "SELECT id, filename, mime_type, size_bytes, created_at
             FROM attachments
             WHERE project_id = $1 AND stage_position = $2
             ORDER BY created_at",
        )
        .bind(self.project_id)
        .bind(self.stage_position)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| self.summary_from_row(row))
            .collect())
    }

    async fn upload(
        &self,
        filename: String,
        mime_type: String,
        data: Vec<u8>,
    ) -> Result<Uuid, BoxError> {
        let size_bytes = data.len() as i64;
        let id = Uuid::new_v4();

        self.storage
            .upload(&id.to_string(), data, &mime_type, &filename)
            .await?;

        let row: (Uuid,) = sqlx::query_as(
            "INSERT INTO attachments(project_id, stage_position, filename, mime_type, size_bytes)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id",
        )
        .bind(self.project_id)
        .bind(self.stage_position)
        .bind(&filename)
        .bind(&mime_type)
        .bind(size_bytes)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }
}