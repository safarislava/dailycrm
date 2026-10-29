use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct PaymentConfirmation {
    pool: Arc<PgPool>,
    stage_id: StageId,
    kind: &'static str,
    confirmed: bool,
}

impl PaymentConfirmation {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, kind: &'static str, confirmed: bool) -> Self {
        Self {
            pool,
            stage_id,
            kind,
            confirmed,
        }
    }
}

#[async_trait::async_trait]
impl Task for PaymentConfirmation {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        sqlx::query(
            "INSERT INTO stage_payments (project_id, parent_position, stage_position, kind, cost, confirmed) \
             VALUES ($1, $2, $3, $4, NULL, $5) \
             ON CONFLICT (project_id, parent_position, stage_position, kind) \
             DO UPDATE SET confirmed = EXCLUDED.confirmed",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
            .bind(self.kind)
            .bind(self.confirmed)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
