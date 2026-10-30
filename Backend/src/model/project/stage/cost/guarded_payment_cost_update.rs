use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::stage::cost::payment_confirmation_sync::PaymentConfirmationSync;
use crate::model::project::stage::cost::payment_cost_update::PaymentCostUpdate;
use crate::model::project::stage::cost::tranche::TrancheError;
use crate::model::project::stage::cost::tranche_total_receipt::TrancheTotalReceipt;
use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct GuardedPaymentCostUpdate {
    pool: Arc<PgPool>,
    stage_id: StageId,
    kind: &'static str,
    cost: Option<ValidCost>,
}

impl GuardedPaymentCostUpdate {
    pub fn new(
        pool: Arc<PgPool>,
        stage_id: StageId,
        kind: &'static str,
        cost: Option<ValidCost>,
    ) -> Self {
        Self {
            pool,
            stage_id,
            kind,
            cost,
        }
    }
}

#[async_trait::async_trait]
impl Task for GuardedPaymentCostUpdate {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let current_total = TrancheTotalReceipt::new(self.pool.clone(), self.stage_id, self.kind)
            .value()
            .await?;

        if let Some(cost) = self.cost {
            let cost_dec = cost.as_decimal()?;
            if cost_dec < current_total {
                return Err(TrancheError::CostBelowPaid.into());
            }
        }

        PaymentCostUpdate::new(self.pool.clone(), self.stage_id, self.kind, self.cost)
            .perform()
            .await?;

        PaymentConfirmationSync::new(self.pool.clone(), self.stage_id, self.kind)
            .perform()
            .await?;

        Ok(())
    }
}
