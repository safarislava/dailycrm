use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::gip_confirmation::GipConfirmation;
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
        GipConfirmation::new(self.pool.clone(), self.stage.clone(), self.confirmed)
            .done()
            .await?;
        if old != Some(self.confirmed) {
            let text = if self.confirmed {
                "ГИП подтвердил выполнение".to_string()
            } else {
                "ГИП снял подтверждение выполнения".to_string()
            };
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
