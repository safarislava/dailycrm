use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::cost::payment_confirmation_sync::PaymentConfirmationSync;
use crate::model::project::stage::cost::tranche::TrancheError;
use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct TrancheDeletion {
    pool: Arc<PgPool>,
    stage_id: StageId,
    kind: &'static str,
    tranche_id: Uuid,
}

impl TrancheDeletion {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, kind: &'static str, tranche_id: Uuid) -> Self {
        Self {
            pool,
            stage_id,
            kind,
            tranche_id,
        }
    }
}

#[async_trait::async_trait]
impl Task for TrancheDeletion {
    type Output = ValidCost;

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct DeletedRow {
            amount: Decimal,
        }

        let deleted = sqlx::query_as::<_, DeletedRow>(
            "DELETE FROM stage_payment_tranches \
             WHERE id = $1 AND project_id = $2 AND parent_position = $3 AND stage_position = $4 AND kind = $5 \
             RETURNING amount",
        )
            .bind(self.tranche_id)
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
            .bind(self.kind)
            .fetch_optional(self.pool.as_ref())
            .await?;

        let deleted_amount = match deleted {
            Some(row) => ValidCost::new(row.amount),
            None => return Err(TrancheError::TrancheNotFound.into()),
        };

        PaymentConfirmationSync::new(self.pool.clone(), self.stage_id, self.kind)
            .perform()
            .await?;

        Ok(deleted_amount)
    }
}
