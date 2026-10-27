use crate::model::contract::box_error::BoxError;
use crate::model::contract::value::Value;
use crate::model::project::stage::cost::contract::cost::Cost;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageFinalCostReceipt {
    pool: Arc<PgPool>,
    stage_id: StageId,
}

impl StageFinalCostReceipt {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId) -> Self {
        Self { pool, stage_id }
    }
}

#[async_trait::async_trait]
impl Value<Option<Box<dyn Cost>>> for StageFinalCostReceipt {
    async fn value(&self) -> Result<Option<Box<dyn Cost>>, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            final_cost: Option<Box<dyn Cost>>,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT final_cost FROM stages WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.and_then(|r| r.final_cost))
    }
}
