use crate::common::BoxError;
use crate::model::project::contract::file::File;
use crate::model::project::file_content::FileContent;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::act_upload_text::ActUploadText;
use crate::model::task::project::comment_text::CommentText;
use crate::model::task::project::notified_act_upload::NotifiedActUpload;
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
    file: FileContent,
}

impl LoggedActUpload {
    pub fn new(
        pool: Arc<PgPool>,
        storage: Arc<Storage>,
        stage: Stage,
        user: User,
        file: FileContent,
    ) -> Self {
        Self {
            pool,
            storage,
            stage,
            user,
            file,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedActUpload {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        NotifiedActUpload::new(
            self.pool.clone(),
            self.storage.clone(),
            self.stage.clone(),
            self.file.clone(),
        )
        .done()
        .await?;
        let text = ActUploadText::new(self.file.name().to_string()).text();
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