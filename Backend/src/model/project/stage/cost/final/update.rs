use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct FinalCostUpdate {
    pool: Arc<PgPool>,
    stage_id: StageId,
    cost: Option<ValidCost>,
}

impl FinalCostUpdate {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, cost: Option<ValidCost>) -> Self {
        Self {
            pool,
            stage_id,
            cost,
        }
    }
}

#[async_trait::async_trait]
impl Task for FinalCostUpdate {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        sqlx::query(
            "INSERT INTO stage_payments (project_id, parent_position, stage_position, kind, cost, confirmed) \
             VALUES ($1, $2, $3, 'final', $4, FALSE) \
             ON CONFLICT (project_id, parent_position, stage_position, kind) \
             DO UPDATE SET cost = EXCLUDED.cost",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
            .bind(self.cost.map(|c| c.as_decimal()).transpose()?)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
