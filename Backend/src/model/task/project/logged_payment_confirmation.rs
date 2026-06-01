use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::payment_confirmation::PaymentConfirmation;
use crate::model::task::project::stage_payment_confirmed_receipt::StagePaymentConfirmedReceipt;
use crate::model::task::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedPaymentConfirmation {
    pool: Arc<PgPool>,
    stage: Stage,
    user: User,
    confirmed: bool,
}

impl LoggedPaymentConfirmation {
    pub fn new(pool: Arc<PgPool>, stage: Stage, user: User, confirmed: bool) -> Self {
        Self { pool, stage, user, confirmed }
    }
}

#[async_trait::async_trait]
impl Task for LoggedPaymentConfirmation {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let old = StagePaymentConfirmedReceipt::new(self.pool.clone(), self.stage.clone())
            .done()
            .await?;
        PaymentConfirmation::new(self.pool.clone(), self.stage.clone(), self.confirmed)
            .done()
            .await?;
        if old != Some(self.confirmed) {
            let text = if self.confirmed {
                "Оплата подтверждена".to_string()
            } else {
                "Подтверждение оплаты снято".to_string()
            };
            let _ = SystemCommentCreation::new(
                self.pool.clone(), self.stage.clone(), self.user.clone(), text,
            )
            .done()
            .await;
        }
        Ok(())
    }
}