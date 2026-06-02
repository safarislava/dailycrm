use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::comment_text::CommentText;
use crate::model::task::project::gip_confirmation_text::GipConfirmationText;
use crate::model::task::project::notified_gip_confirmation::NotifiedGipConfirmation;
use crate::model::task::project::stage_gip_confirmed_receipt::StageGipConfirmedReceipt;
use crate::model::task::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedGipConfirmation {
    pool: Arc<PgPool>,
    stage: Stage,
    user: User,
    confirmed: bool,
}

impl LoggedGipConfirmation {
    pub fn new(pool: Arc<PgPool>, stage: Stage, user: User, confirmed: bool) -> Self {
        Self {
            pool,
            stage,
            user,
            confirmed,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedGipConfirmation {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let old = StageGipConfirmedReceipt::new(self.pool.clone(), self.stage.clone())
            .done()
            .await?;
        NotifiedGipConfirmation::new(self.pool.clone(), self.stage.clone(), self.confirmed)
            .done()
            .await?;
        if old != Some(self.confirmed) {
            let text = GipConfirmationText::new(self.confirmed).text();
            let _ = SystemCommentCreation::new(
                self.pool.clone(),
                self.stage.clone(),
                self.user.clone(),
                text,
            )
            .done()
            .await;
        }
        Ok(())
    }
}