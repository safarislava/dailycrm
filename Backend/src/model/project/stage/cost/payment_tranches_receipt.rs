use crate::model::contract::box_error::BoxError;
use crate::model::contract::value::Value;
use crate::model::project::stage::cost::tranche::StagePaymentTranche;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct PaymentTranchesReceipt {
    pool: Arc<PgPool>,
    stage_id: StageId,
    kind: &'static str,
}

impl PaymentTranchesReceipt {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, kind: &'static str) -> Self {
        Self {
            pool,
            stage_id,
            kind,
        }
    }
}

#[async_trait::async_trait]
impl Value<Vec<StagePaymentTranche>> for PaymentTranchesReceipt {
    async fn value(&self) -> Result<Vec<StagePaymentTranche>, BoxError> {
        let tranches = sqlx::query_as::<_, StagePaymentTranche>(
            "SELECT id, amount, created_at, created_by \
             FROM stage_payment_tranches \
             WHERE project_id = $1 AND parent_position = $2 AND stage_position = $3 AND kind = $4 \
             ORDER BY created_at",
        )
        .bind(self.stage_id.project_id().id())
        .bind(self.stage_id.parent_position())
        .bind(self.stage_id.position())
        .bind(self.kind)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(tranches)
    }
}
