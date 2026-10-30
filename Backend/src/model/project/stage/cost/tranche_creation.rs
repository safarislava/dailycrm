use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::cost::payment_confirmation_sync::PaymentConfirmationSync;
use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct TrancheCreation {
    pool: Arc<PgPool>,
    stage_id: StageId,
    kind: &'static str,
    amount: ValidCost,
    created_by: Option<Uuid>,
}

impl TrancheCreation {
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
impl Task for TrancheCreation {
    type Output = Uuid;

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let tranche_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO stage_payment_tranches (id, project_id, parent_position, stage_position, kind, amount, created_at, created_by) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW(), $7)",
        )
            .bind(tranche_id)
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
            .bind(self.kind)
            .bind(self.amount.as_decimal()?)
            .bind(self.created_by)
            .execute(self.pool.as_ref())
            .await?;

        PaymentConfirmationSync::new(self.pool.clone(), self.stage_id, self.kind)
            .perform()
            .await?;

        Ok(tranche_id)
    }
}
