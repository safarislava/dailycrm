use crate::storage::Storage;
use sqlx::PgPool;
use uuid::Uuid;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct AttachmentLink {
    id: Uuid,
    project_id: Uuid,
}

impl AttachmentLink {
    pub fn new(id: Uuid, project_id: Uuid) -> Self {
        Self { id, project_id }
    }

    pub async fn delete(self, pool: &PgPool, storage: &Storage) -> Result<(), BoxError> {
        let _ = storage.delete(&self.id.to_string()).await;
        let result = sqlx::query("DELETE FROM attachments WHERE id = $1 AND project_id = $2")
            .bind(self.id)
            .bind(self.project_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err("attachment not found".into());
        }
        Ok(())
    }
}