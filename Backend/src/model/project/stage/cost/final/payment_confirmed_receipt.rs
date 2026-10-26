use crate::model::contract::box_error::BoxError;
use crate::model::contract::value::Value;
use crate::model::project::stage::id::StageId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageFinalPaymentConfirmedReceipt {
    pool: Arc<PgPool>,
    stage_id: StageId,
}

impl StageFinalPaymentConfirmedReceipt {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId) -> Self {
        Self { pool, stage_id }
    }
}

#[async_trait::async_trait]
impl Value<Option<bool>> for StageFinalPaymentConfirmedReceipt {
    async fn value(&self) -> Result<Option<bool>, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            final_confirmed: bool,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT confirmed AS final_confirmed FROM stage_payments WHERE project_id = $1 AND parent_position = $2 AND stage_position = $3 AND kind = 'final'",
        )
            .bind(self.stage_id.project_id().id())
            .bind(self.stage_id.parent_position())
            .bind(self.stage_id.position())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.map(|r| r.final_confirmed))
    }
}
