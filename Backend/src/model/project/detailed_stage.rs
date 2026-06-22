use crate::common::BoxError;
use crate::model::project::contract::json::Json;
use crate::model::project::stage::Stage;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct DetailedStage {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl DetailedStage {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        DetailedStage { pool, stage }
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
            advance_cost: Option<i32>,
            advance_confirmed: bool,
            final_cost: Option<i32>,
            final_confirmed: bool,
            gip_confirmed: bool,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT project_id, parent_position, position, title, deadline,
                    completed, advance_cost, advance_confirmed, final_cost, final_confirmed, gip_confirmed
             FROM detailed_stages WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(serde_json::to_value(row)?)
    }
}