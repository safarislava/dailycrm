use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::comment_text::CommentText;
use crate::model::task::project::rename_text::RenameText;
use crate::model::task::project::stage_rename::StageRename;
use crate::model::task::project::stage_title_receipt::StageTitleReceipt;
use crate::model::task::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedStageRename {
    pool: Arc<PgPool>,
    stage: Stage,
    user: User,
    title: String,
}

impl LoggedStageRename {
    pub fn new(pool: Arc<PgPool>, stage: Stage, user: User, title: String) -> Self {
        Self {
            pool,
            stage,
            user,
            title,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedStageRename {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let old = StageTitleReceipt::new(self.pool.clone(), self.stage.clone())
            .done()
            .await?;
        StageRename::new(self.pool.clone(), self.stage.clone(), self.title.clone())
            .done()
            .await?;
        if let Some(old_title) = old {
            if old_title != self.title {
                let text = RenameText::new(old_title, self.title.clone()).text();
                let _ = SystemCommentCreation::new(
                    self.pool.clone(),
                    self.stage.clone(),
                    self.user.clone(),
                    text,
                )
                .done()
                .await;
            }
        }
        Ok(())
    }
}