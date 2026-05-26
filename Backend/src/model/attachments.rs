use crate::model::attachment::Attachment;
use crate::model::attachment_link::AttachmentLink;
use crate::storage::Storage;
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

pub struct Attachments {
    project_id: Uuid,
    stage_position: i32,
}

impl Attachments {
    pub fn new(project_id: Uuid, stage_position: i32) -> Self {
        Self { project_id, stage_position }
    }

    pub fn attachment_link(&self, attachment_id: Uuid) -> AttachmentLink {
        AttachmentLink::new(attachment_id, self.project_id)
    }

    fn attachment_from_row(&self, row: AttachmentRow) -> Attachment {
        let url = format!(
            "/api/projects/{}/stages/{}/attachments/{}/download",
            self.project_id, self.stage_position, row.id
        );
        Attachment::new(row.id, row.filename, row.mime_type, row.size_bytes, row.created_at, url)
    }

    pub async fn list(&self, pool: &PgPool) -> Result<Vec<Attachment>, sqlx::Error> {
        let rows = sqlx::query_as::<_, AttachmentRow>(
            "SELECT id, filename, mime_type, size_bytes, created_at
             FROM attachments
             WHERE project_id = $1 AND stage_position = $2
             ORDER BY created_at",
        )
        .bind(self.project_id)
        .bind(self.stage_position)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|row| self.attachment_from_row(row)).collect())
    }

    pub async fn attachment_by_id(&self, attachment_id: Uuid, pool: &PgPool) -> Result<Attachment, sqlx::Error> {
        let row = sqlx::query_as::<_, AttachmentRow>(
            "SELECT id, filename, mime_type, size_bytes, created_at
             FROM attachments
             WHERE id = $1 AND project_id = $2 AND stage_position = $3",
        )
        .bind(attachment_id)
        .bind(self.project_id)
        .bind(self.stage_position)
        .fetch_one(pool)
        .await?;

        Ok(self.attachment_from_row(row))
    }

    pub async fn upload(
        &self,
        filename: String,
        mime_type: String,
        data: Vec<u8>,
        pool: &PgPool,
        storage: &Storage,
    ) -> Result<Uuid, BoxError> {
        let size_bytes = data.len() as i64;
        let attachment_id = Uuid::new_v4();

        storage.upload(&attachment_id.to_string(), data, &mime_type, &filename).await?;

        let row: (Uuid,) = sqlx::query_as(
            "INSERT INTO attachments(project_id, stage_position, filename, mime_type, size_bytes)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id",
        )
        .bind(self.project_id)
        .bind(self.stage_position)
        .bind(filename)
        .bind(mime_type)
        .bind(size_bytes)
        .fetch_one(pool)
        .await?;

        Ok(row.0)
    }
}