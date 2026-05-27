use crate::model::attachment::Attachment;
use crate::model::attachment_link::AttachmentLink;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

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
    pool: PgPool,
}

impl Attachments {
    pub fn new(project_id: Uuid, stage_position: i32, pool: PgPool) -> Self {
        Self { project_id, stage_position, pool }
    }

    pub fn attachment_link(&self, attachment_id: Uuid) -> AttachmentLink {
        AttachmentLink::new(attachment_id, self.project_id, self.pool.clone())
    }

    pub async fn list(&self) -> Result<Vec<Attachment>, sqlx::Error> {
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

        Ok(rows.into_iter().map(|row| self.attachment_from_row(row)).collect())
    }

    pub async fn by_id(&self, attachment_id: Uuid) -> Result<Attachment, sqlx::Error> {
        let row = sqlx::query_as::<_, AttachmentRow>(
            "SELECT id, filename, mime_type, size_bytes, created_at
             FROM attachments
             WHERE id = $1 AND project_id = $2 AND stage_position = $3",
        )
        .bind(attachment_id)
        .bind(self.project_id)
        .bind(self.stage_position)
        .fetch_one(&self.pool)
        .await?;

        Ok(self.attachment_from_row(row))
    }

    fn attachment_from_row(&self, row: AttachmentRow) -> Attachment {
        let url = format!(
            "/api/projects/{}/stages/{}/attachments/{}/download",
            self.project_id, self.stage_position, row.id
        );
        Attachment::new(
            row.id,
            row.filename,
            row.mime_type,
            row.size_bytes,
            row.created_at,
            url,
        )
    }
}