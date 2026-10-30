use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::stage::comment::system_creation::SystemCommentCreation;
use crate::model::project::stage::cost::guarded_payment_cost_update::GuardedPaymentCostUpdate;
use crate::model::project::stage::cost::stage_payment_cost_receipt::StagePaymentCostReceipt;
use crate::model::project::stage::cost::text::PaymentCostText;
use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedPaymentCostUpdate<T: PaymentCostText> {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    kind: &'static str,
    cost: Option<ValidCost>,
    text: T,
}

impl<T: PaymentCostText> LoggedPaymentCostUpdate<T> {
    pub fn new(
        pool: Arc<PgPool>,
        stage_id: StageId,
        user_id: UserId,
        kind: &'static str,
        cost: Option<ValidCost>,
        text: T,
    ) -> Self {
        Self {
            pool,
            stage_id,
            user_id,
            kind,
            cost,
            text,
        }
    }
}

#[async_trait::async_trait]
impl<T: PaymentCostText> Task for LoggedPaymentCostUpdate<T> {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old = StagePaymentCostReceipt::new(self.pool.clone(), self.stage_id, self.kind)
            .value()
            .await?;
        GuardedPaymentCostUpdate::new(self.pool.clone(), self.stage_id, self.kind, self.cost)
            .perform()
            .await?;
        if let Some(old_cost) = old
            && self.cost != Some(old_cost)
        {
            let text = self.text.text(old_cost, self.cost);
            let _ =
                SystemCommentCreation::new(self.pool.clone(), self.stage_id, self.user_id, text)
                    .perform()
                    .await;
        }
        Ok(())
    }
}
