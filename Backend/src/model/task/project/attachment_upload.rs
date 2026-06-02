use crate::common::BoxError;
use crate::model::project::contract::file::File;
use crate::model::project::file_content::FileContent;
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
    file: FileContent,
}

impl AttachmentUpload {
    pub fn new(pool: Arc<PgPool>, storage: Arc<Storage>, stage: Stage, file: FileContent) -> Self {
        Self {
            pool,
            storage,
            stage,
            file,
        }
    }
}

#[async_trait::async_trait]
impl Task for AttachmentUpload {
    type Output = Uuid;

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let id = Uuid::new_v4();
        self.file
            .upload_to(self.storage.as_ref(), &id.to_string())
            .await?;
        sqlx::query(
            "INSERT INTO attachments(id, project_id, stage_position, filename, mime_type, size_bytes, is_act)
             VALUES ($1, $2, $3, $4, $5, $6, false)",
        )
        .bind(id)
        .bind(self.stage.project().id())
        .bind(self.stage.position())
        .bind(self.file.name())
        .bind(self.file.media_type())
        .bind(self.file.size_bytes())
        .execute(self.pool.as_ref())
        .await?;
        Ok(id)
    }
}