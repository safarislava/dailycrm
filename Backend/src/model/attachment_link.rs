use crate::storage::Storage;
use sqlx::PgPool;
use uuid::Uuid;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct AttachmentLink {
    id: Uuid,
    project_id: Uuid,
    pool: PgPool,
    storage: Storage,
}

impl AttachmentLink {
    pub fn new(id: Uuid, project_id: Uuid, pool: PgPool, storage: Storage) -> Self {
        Self { id, project_id, pool, storage }
    }

    pub async fn delete(self) -> Result<(), BoxError> {
        let _ = self.storage.delete(&self.id.to_string()).await;
        let result = sqlx::query(
            "DELETE FROM attachments WHERE id = $1 AND project_id = $2",
        )
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