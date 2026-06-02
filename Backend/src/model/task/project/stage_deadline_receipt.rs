use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageDeadlineReceipt {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl StageDeadlineReceipt {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl Task for StageDeadlineReceipt {
    type Output = Option<DateTime<Utc>>;

    async fn done(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            deadline: Option<DateTime<Utc>>,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT deadline FROM stages WHERE project_id = $1 AND position = $2",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.position())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.and_then(|r| r.deadline))
    }
}
