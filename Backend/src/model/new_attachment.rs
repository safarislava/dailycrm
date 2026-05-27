use crate::storage::Storage;
use sqlx::PgPool;
use uuid::Uuid;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct NewAttachment {
    project_id: Uuid,
    stage_position: i32,
    filename: String,
    mime_type: String,
    data: Vec<u8>,
    pool: PgPool,
    storage: Storage,
}

impl NewAttachment {
    pub fn new(
        project_id: Uuid,
        stage_position: i32,
        filename: String,
        mime_type: String,
        data: Vec<u8>,
        pool: PgPool,
        storage: Storage,
    ) -> Self {
        Self { project_id, stage_position, filename, mime_type, data, pool, storage }
    }

    pub async fn save(self) -> Result<Uuid, BoxError> {
        let size_bytes = self.data.len() as i64;
        let id = Uuid::new_v4();

        self.storage
            .upload(&id.to_string(), self.data, &self.mime_type, &self.filename)
            .await?;

        let row: (Uuid,) = sqlx::query_as(
            "INSERT INTO attachments(project_id, stage_position, filename, mime_type, size_bytes)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id",
        )
        .bind(self.project_id)
        .bind(self.stage_position)
        .bind(self.filename)
        .bind(self.mime_type)
        .bind(size_bytes)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }
}