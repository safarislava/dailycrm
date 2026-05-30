use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct CompletionUpdate {
    pool: Arc<PgPool>,
    stage: Stage,
    completed: bool,
}

impl CompletionUpdate {
    pub fn new(pool: Arc<PgPool>, stage: Stage, completed: bool) -> Self {
        Self {
            pool,
            stage,
            completed,
        }
    }
}

#[async_trait::async_trait]
impl Task for CompletionUpdate {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("UPDATE stages SET completed = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.stage.project().id())
            .bind(self.stage.position())
            .bind(self.completed)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
