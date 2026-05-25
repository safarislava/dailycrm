use crate::model::attachment::Attachment;
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
    object_key: String,
    created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Attachments {
    pool: PgPool,
    storage: Storage,
}

impl Attachments {
    pub fn new(pool: PgPool, storage: Storage) -> Self {
        Self { pool, storage }
    }

    pub async fn list(
        &self,
        project_id: Uuid,
        stage_position: i32,
    ) -> Result<Vec<Attachment>, sqlx::Error> {
        let rows = sqlx::query_as::<_, AttachmentRow>(
            "SELECT id, filename, mime_type, size_bytes, object_key, created_at
             FROM attachments
             WHERE project_id = $1 AND stage_position = $2
             ORDER BY created_at",
        )
        .bind(project_id)
        .bind(stage_position)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let url = format!(
                    "/api/projects/{}/stages/{}/attachments/{}/download",
                    project_id, stage_position, row.id
                );
                Attachment::new(
                    row.id,
                    row.filename,
                    row.mime_type,
                    row.size_bytes,
                    row.created_at,
                    url,
                )
            })
            .collect())
    }

    pub async fn download(
        &self,
        project_id: Uuid,
        stage_position: i32,
        attachment_id: Uuid,
    ) -> Result<(Vec<u8>, String, String), BoxError> {
        let row: (String, String, String) = sqlx::query_as(
            "SELECT object_key, mime_type, filename
             FROM attachments
             WHERE id = $1 AND project_id = $2 AND stage_position = $3",
        )
        .bind(attachment_id)
        .bind(project_id)
        .bind(stage_position)
        .fetch_one(&self.pool)
        .await?;

        let (object_key, mime_type, filename) = row;
        let data = self.storage.get_bytes(&object_key).await?;
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

    pub async fn upload(
        &self,
        project_id: Uuid,
        stage_position: i32,
        filename: String,
        mime_type: String,
        data: Vec<u8>,
    ) -> Result<Uuid, BoxError> {
        let size_bytes = data.len() as i64;
        let attachment_id = Uuid::new_v4();
        let object_key = format!(
            "{}/{}/{}/{}",
            project_id, stage_position, attachment_id, filename
        );

        self.storage
            .upload(&object_key, data, &mime_type, &filename)
            .await?;

        let row: (Uuid,) = sqlx::query_as(
            "INSERT INTO attachments(project_id, stage_position, filename, mime_type, size_bytes, object_key)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id",
        )
        .bind(project_id)
        .bind(stage_position)
        .bind(filename)
        .bind(mime_type)
        .bind(size_bytes)
        .bind(object_key)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }

    pub async fn delete(&self, attachment_id: Uuid, project_id: Uuid) -> Result<(), BoxError> {
        let row: (String,) =
            sqlx::query_as("SELECT object_key FROM attachments WHERE id = $1 AND project_id = $2")
                .bind(attachment_id)
                .bind(project_id)
                .fetch_one(&self.pool)
                .await?;

        let _ = self.storage.delete(&row.0).await;

        sqlx::query("DELETE FROM attachments WHERE id = $1 AND project_id = $2")
            .bind(attachment_id)
            .bind(project_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
