use crate::model::contract::box_error::BoxError;
use crate::model::contract::value::Value;
use crate::model::project::stage::id::StageId;
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::sync::Arc;

pub struct TrancheTotalReceipt {
    pool: Arc<PgPool>,
    stage_id: StageId,
    kind: &'static str,
}

impl TrancheTotalReceipt {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId, kind: &'static str) -> Self {
        Self {
            pool,
            stage_id,
            kind,
        }
    }
}

#[async_trait::async_trait]
impl Value<Decimal> for TrancheTotalReceipt {
    async fn value(&self) -> Result<Decimal, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            total: Option<Decimal>,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT COALESCE(SUM(amount), 0) AS total \
             FROM stage_payment_tranches \
             WHERE project_id = $1 AND parent_position = $2 AND stage_position = $3 AND kind = $4",
        )
        .bind(self.stage_id.project_id().id())
        .bind(self.stage_id.parent_position())
        .bind(self.stage_id.position())
        .bind(self.kind)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row.total.unwrap_or_default())
    }
}
