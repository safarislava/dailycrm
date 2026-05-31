use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::attachment_upload::AttachmentUpload;
use crate::model::task::project::notification_enqueue::NotificationEnqueue;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct ActUpload {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    stage: Stage,
    filename: String,
    mime_type: String,
    data: Vec<u8>,
}

impl ActUpload {
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
impl Task for ActUpload {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        AttachmentUpload::new(
            self.pool.clone(),
            self.storage.clone(),
            self.stage.clone(),
            self.filename.clone(),
            self.mime_type.clone(),
            self.data.clone(),
            true,
        )
        .done()
        .await?;
        NotificationEnqueue::new(self.pool.clone(), self.stage.clone(), "act_uploaded")
            .done()
            .await
    }
}
