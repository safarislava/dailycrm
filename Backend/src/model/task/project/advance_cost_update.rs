use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AdvanceCostUpdate {
    pool: Arc<PgPool>,
    stage: Stage,
    cost: Option<i32>,
}

impl AdvanceCostUpdate {
    pub fn new(pool: Arc<PgPool>, stage: Stage, cost: Option<i32>) -> Self {
        Self { pool, stage, cost }
    }
}

#[async_trait::async_trait]
impl Task for AdvanceCostUpdate {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("UPDATE stages SET advance_cost = $4 WHERE project_id = $1 AND parent_position = $2 AND position = $3")
            .bind(self.stage.project().id())
            .bind(self.stage.parent_position())
            .bind(self.stage.position())
            .bind(self.cost)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}