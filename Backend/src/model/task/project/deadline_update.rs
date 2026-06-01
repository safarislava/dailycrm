use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;

pub struct DeadlineUpdate {
    pool: Arc<PgPool>,
    stage: Stage,
    deadline: Option<DateTime<Utc>>,
}

impl DeadlineUpdate {
    pub fn new(pool: Arc<PgPool>, stage: Stage, deadline: Option<DateTime<Utc>>) -> Self {
        Self { pool, stage, deadline }
    }
}

#[async_trait::async_trait]
impl Task for DeadlineUpdate {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("UPDATE stages SET deadline = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.stage.project().id())
            .bind(self.stage.position())
            .bind(self.deadline)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}