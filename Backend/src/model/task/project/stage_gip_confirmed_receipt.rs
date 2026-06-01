use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageGipConfirmedReceipt {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl StageGipConfirmedReceipt {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl Task for StageGipConfirmedReceipt {
    type Output = Option<bool>;

    async fn done(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row { gip_confirmed: bool }
        let row = sqlx::query_as::<_, Row>(
            "SELECT gip_confirmed FROM stages WHERE project_id = $1 AND position = $2",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.position())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.map(|r| r.gip_confirmed))
    }
}