use crate::common::BoxError;
use crate::model::project::file_content::FileContent;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::notification::notification_enqueue::NotificationEnqueue;
use crate::model::task::project::act_upload::ActUpload;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotifiedActUpload {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    stage: Stage,
    file: FileContent,
}

impl NotifiedActUpload {
    pub fn new(
        pool: Arc<PgPool>,
        storage: Arc<Storage>,
        stage: Stage,
        file: FileContent,
    ) -> Self {
        Self {
            pool,
            storage,
            stage,
            file,
        }
    }
}

#[async_trait::async_trait]
impl Task for NotifiedActUpload {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        ActUpload::new(
            self.pool.clone(),
            self.storage.clone(),
            self.stage.clone(),
            self.file.clone(),
        )
        .done()
        .await?;
        NotificationEnqueue::new(self.pool.clone(), self.stage.clone(), "act_uploaded")
            .done()
            .await
    }
}