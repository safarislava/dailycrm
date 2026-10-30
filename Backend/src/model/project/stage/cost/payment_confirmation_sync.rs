use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct PaymentConfirmationSync {
    pool: Arc<PgPool>,
    stage_id: StageId,
    kind: &'static str,
}

impl PaymentConfirmationSync {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, kind: &'static str) -> Self {
        Self {
            pool,
            stage_id,
            kind,
        }
    }
}

#[async_trait::async_trait]
impl Task for PaymentConfirmationSync {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        sqlx::query(
            "UPDATE stage_payments \
             SET confirmed = (COALESCE((SELECT SUM(amount) FROM stage_payment_tranches WHERE project_id = $1 AND parent_position = $2 AND stage_position = $3 AND kind = $4), 0) >= cost AND cost > 0) \
             WHERE project_id = $1 AND parent_position = $2 AND stage_position = $3 AND kind = $4",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
            .bind(self.kind)
            .execute(self.pool.as_ref())
            .await?;

        Ok(())
    }
}
