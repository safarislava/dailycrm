use crate::model::contract::box_error::BoxError;
use crate::model::project::contract::json::Json;
use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct DetailedStage {
    pool: Arc<PgPool>,
    stage_id: StageId,
}

impl DetailedStage {
    pub fn new(pool: Arc<PgPool>, stage_id: StageId) -> Self {
        DetailedStage { pool, stage_id }
    }
}

#[async_trait::async_trait]
impl Json for DetailedStage {
    async fn json(&self) -> Result<serde_json::Value, BoxError> {
        #[derive(sqlx::FromRow, Serialize)]
        struct Row {
            project_id: Uuid,
            parent_position: i32,
            position: i32,
            title: String,
            deadline: Option<DateTime<Utc>>,
            completed: bool,
            advance_cost: Option<ValidCost>,
            advance_confirmed: bool,
            second_advance_cost: Option<ValidCost>,
            second_advance_confirmed: bool,
            final_cost: Option<ValidCost>,
            final_confirmed: bool,
            gip_confirmed: bool,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT project_id, parent_position, position, title, deadline,
                    completed, advance_cost, advance_confirmed,
                    second_advance_cost, second_advance_confirmed,
                    final_cost, final_confirmed, gip_confirmed
             FROM detailed_stages WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
        .bind(self.stage_id.project_id().id())
        .bind(self.stage_id.parent_position())
        .bind(self.stage_id.position())
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(serde_json::to_value(row)?)
    }
}
