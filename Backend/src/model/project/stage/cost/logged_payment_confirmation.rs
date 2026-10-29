use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::stage::comment::system_creation::SystemCommentCreation;
use crate::model::project::stage::cost::payment_confirmation::PaymentConfirmation;
use crate::model::project::stage::cost::stage_payment_confirmed_receipt::StagePaymentConfirmedReceipt;
use crate::model::project::stage::cost::text::PaymentConfirmationText;
use crate::model::project::stage::id::StageId;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedPaymentConfirmation<T: PaymentConfirmationText> {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    kind: &'static str,
    confirmed: bool,
    text: T,
}

impl<T: PaymentConfirmationText> LoggedPaymentConfirmation<T> {
    pub fn new(
        pool: Arc<PgPool>,
        stage_id: StageId,
        user_id: UserId,
        kind: &'static str,
        confirmed: bool,
        text: T,
    ) -> Self {
        Self {
            pool,
            stage_id,
            user_id,
            kind,
            confirmed,
            text,
        }
    }
}

#[async_trait::async_trait]
impl<T: PaymentConfirmationText> Task for LoggedPaymentConfirmation<T> {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old = StagePaymentConfirmedReceipt::new(self.pool.clone(), self.stage_id, self.kind)
            .value()
            .await?;
        PaymentConfirmation::new(self.pool.clone(), self.stage_id, self.kind, self.confirmed)
            .perform()
            .await?;
        if old != Some(self.confirmed) {
            let text = self.text.text(self.confirmed);
            let _ =
                SystemCommentCreation::new(self.pool.clone(), self.stage_id, self.user_id, text)
                    .perform()
                    .await;
        }
        Ok(())
    }
}
