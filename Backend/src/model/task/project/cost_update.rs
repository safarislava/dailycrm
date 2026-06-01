use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct CostUpdate {
    pool: Arc<PgPool>,
    stage: Stage,
    cost: Option<i32>,
}

impl CostUpdate {
    pub fn new(pool: Arc<PgPool>, stage: Stage, cost: Option<i32>) -> Self {
        Self { pool, stage, cost }
    }
}

#[async_trait::async_trait]
impl Task for CostUpdate {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("UPDATE stages SET cost = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.stage.project().id())
            .bind(self.stage.position())
            .bind(self.cost)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}