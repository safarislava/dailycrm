use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::cost::contract::cost::Cost;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AdvanceCostUpdate {
    pool: Arc<PgPool>,
    stage_id: StageId,
    cost: Option<Box<dyn Cost>>,
}

impl AdvanceCostUpdate {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, cost: Option<Box<dyn Cost>>) -> Self {
        Self {
            pool,
            stage_id,
            cost,
        }
    }
}

#[async_trait::async_trait]
impl Task for AdvanceCostUpdate {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("UPDATE stages SET advance_cost = $4 WHERE project_id = $1 AND parent_position = $2 AND position = $3")
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
            .bind(self.cost.as_ref().map(|c| c.as_decimal()).transpose()?)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
