use crate::model::contract::box_error::BoxError;
use crate::model::contract::value::Value;
use crate::model::project::stage::cost::valid_cost::ValidCost;
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
impl Value<Option<ValidCost>> for StageFinalCostReceipt {
    async fn value(&self) -> Result<Option<ValidCost>, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            final_cost: Option<ValidCost>,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT cost AS final_cost FROM stage_payments WHERE project_id = $1 AND parent_position = $2 AND stage_position = $3 AND kind = 'final'",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.and_then(|r| r.final_cost))
    }
}
