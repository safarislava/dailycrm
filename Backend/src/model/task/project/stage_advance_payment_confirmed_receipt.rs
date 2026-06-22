use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageAdvancePaymentConfirmedReceipt {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl StageAdvancePaymentConfirmedReceipt {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl Task for StageAdvancePaymentConfirmedReceipt {
    type Output = Option<bool>;

    async fn done(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            advance_confirmed: bool,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT advance_confirmed FROM stages WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.map(|r| r.advance_confirmed))
    }
}