use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::attachment_upload::AttachmentUpload;
use crate::model::task::project::attachment_upload_text::AttachmentUploadText;
use crate::model::task::project::comment_text::CommentText;
use crate::model::task::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct LoggedAttachmentUpload {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    stage: Stage,
    user: User,
    filename: String,
    mime_type: String,
    data: Vec<u8>,
}

impl LoggedAttachmentUpload {
    pub fn new(
        pool: Arc<PgPool>,
        storage: Arc<Storage>,
        stage: Stage,
        user: User,
        filename: String,
        mime_type: String,
        data: Vec<u8>,
    ) -> Self {
        Self {
            pool,
            storage,
            stage,
            user,
            filename,
            mime_type,
            data,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedAttachmentUpload {
    type Output = Uuid;

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let id = AttachmentUpload::new(
            self.pool.clone(),
            self.storage.clone(),
            self.stage.clone(),
            self.filename.clone(),
            self.mime_type.clone(),
            self.data.clone(),
            false,
        )
        .done()
        .await?;
        let text = AttachmentUploadText::new(self.filename.clone()).text();
        let _ = SystemCommentCreation::new(
            self.pool.clone(),
            self.stage.clone(),
            self.user.clone(),
            text,
        )
        .done()
        .await;
        Ok(id)
    }
}