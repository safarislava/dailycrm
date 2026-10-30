use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::stage::cost::stage_payment_cost_receipt::StagePaymentCostReceipt;
use crate::model::project::stage::cost::tranche::TrancheError;
use crate::model::project::stage::cost::tranche_creation::TrancheCreation;
use crate::model::project::stage::cost::tranche_total_receipt::TrancheTotalReceipt;
use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct GuardedTrancheCreation {
    pool: Arc<PgPool>,
    stage_id: StageId,
    kind: &'static str,
    amount: ValidCost,
    created_by: Option<Uuid>,
}

impl GuardedTrancheCreation {
    pub fn new(
        pool: Arc<PgPool>,
        stage_id: StageId,
        kind: &'static str,
        amount: ValidCost,
        created_by: Option<Uuid>,
    ) -> Self {
        Self {
            pool,
            stage_id,
            kind,
            amount,
            created_by,
        }
    }
}

#[async_trait::async_trait]
impl Task for GuardedTrancheCreation {
    type Output = Uuid;

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let planned_cost =
            StagePaymentCostReceipt::new(self.pool.clone(), self.stage_id, self.kind)
                .value()
                .await?;

        let planned_decimal = match planned_cost {
            Some(c) => {
                let dec = c.as_decimal()?;
                if dec <= Decimal::ZERO {
                    return Err(TrancheError::CostNotSet.into());
                }
                dec
            }
            None => return Err(TrancheError::CostNotSet.into()),
        };

        let current_total = TrancheTotalReceipt::new(self.pool.clone(), self.stage_id, self.kind)
            .value()
            .await?;

        let tranche_amount = self.amount.as_decimal()?;
        if current_total + tranche_amount > planned_decimal {
            return Err(TrancheError::OverpaymentForbidden.into());
        }

        TrancheCreation::new(
            self.pool.clone(),
            self.stage_id,
            self.kind,
            self.amount,
            self.created_by,
        )
        .perform()
        .await
    }
}
