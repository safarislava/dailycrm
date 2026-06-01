use crate::common::BoxError;
use crate::model::project::attachment::Attachment;
use crate::model::task::contract::task::Task;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AttachmentRemoval {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    attachment: Attachment,
}

impl AttachmentRemoval {
    pub fn new(pool: Arc<PgPool>, storage: Arc<Storage>, attachment: Attachment) -> Self {
        Self { pool, storage, attachment }
    }
}

#[async_trait::async_trait]
impl Task for AttachmentRemoval {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let _ = self.storage.delete(&self.attachment.id().to_string()).await;
        let result = sqlx::query("DELETE FROM attachments WHERE id = $1")
            .bind(self.attachment.id())
            .execute(self.pool.as_ref())
            .await?;
        if result.rows_affected() == 0 {
            return Err(BoxError::from("attachment not found"));
        }
        Ok(())
    }
}