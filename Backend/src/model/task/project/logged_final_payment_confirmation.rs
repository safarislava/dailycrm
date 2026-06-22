use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::comment_text::CommentText;
use crate::model::task::project::final_payment_confirmation::FinalPaymentConfirmation;
use crate::model::task::project::final_payment_confirmation_text::FinalPaymentConfirmationText;
use crate::model::task::project::stage_final_payment_confirmed_receipt::StageFinalPaymentConfirmedReceipt;
use crate::model::task::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedFinalPaymentConfirmation {
    pool: Arc<PgPool>,
    stage: Stage,
    user: User,
    confirmed: bool,
}

impl LoggedFinalPaymentConfirmation {
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
impl Task for LoggedFinalPaymentConfirmation {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let old = StageFinalPaymentConfirmedReceipt::new(self.pool.clone(), self.stage.clone())
            .done()
            .await?;
        FinalPaymentConfirmation::new(self.pool.clone(), self.stage.clone(), self.confirmed)
            .done()
            .await?;
        if old != Some(self.confirmed) {
            let text = FinalPaymentConfirmationText::new(self.confirmed).text();
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