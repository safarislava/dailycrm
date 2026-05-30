use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AttachmentUpload {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    stage: Stage,
    filename: String,
    mime_type: String,
    data: Vec<u8>,
}

impl AttachmentUpload {
    pub fn new(
        pool: Arc<PgPool>,
        storage: Arc<Storage>,
        stage: Stage,
        filename: String,
        mime_type: String,
        data: Vec<u8>,
    ) -> Self {
        Self {
            pool,
            storage,
            stage,
            filename,
            mime_type,
            data,
        }
    }
}

#[async_trait::async_trait]
impl Task for AttachmentUpload {
    type Output = Uuid;

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let size_bytes = self.data.len() as i64;
        let id = Uuid::new_v4();
        self.storage
            .upload(
                &id.to_string(),
                self.data.clone(),
                &self.mime_type,
                &self.filename,
            )
            .await?;
        let row: (Uuid,) = sqlx::query_as(
            "INSERT INTO attachments(project_id, stage_position, filename, mime_type, size_bytes)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.position())
        .bind(&self.filename)
        .bind(&self.mime_type)
        .bind(size_bytes)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row.0)
    }
}
