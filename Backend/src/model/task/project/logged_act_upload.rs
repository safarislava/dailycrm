use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::act_upload::ActUpload;
use crate::model::task::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedActUpload {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    stage: Stage,
    user: User,
    filename: String,
    mime_type: String,
    data: Vec<u8>,
}

impl LoggedActUpload {
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
impl Task for LoggedActUpload {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        ActUpload::new(
            self.pool.clone(),
            self.storage.clone(),
            self.stage.clone(),
            self.filename.clone(),
            self.mime_type.clone(),
            self.data.clone(),
        )
        .done()
        .await?;
        let text = format!("Загружен акт: {}", self.filename);
        let _ = SystemCommentCreation::new(
            self.pool.clone(),
            self.stage.clone(),
            self.user.clone(),
            text,
        )
        .done()
        .await;
        Ok(())
    }
}
