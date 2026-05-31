use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
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
impl Contentable for DetailedStage {
    type Output = serde_json::Value;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow, Serialize)]
        struct Row {
            project_id: Uuid,
            position: i32,
            title: String,
            deadline: Option<DateTime<Utc>>,
            completed: bool,
            cost: Option<i32>,
            gip_confirmed: bool,
            payment_confirmed: bool,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT project_id, position, title, deadline,
                    (gip_confirmed AND payment_confirmed AND EXISTS(
                        SELECT 1 FROM attachments a
                        WHERE a.project_id = stages.project_id
                        AND a.stage_position = stages.position AND a.is_act = TRUE
                    )) AS completed,
                    cost, gip_confirmed, payment_confirmed
             FROM stages WHERE project_id = $1 AND position = $2",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.position())
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(serde_json::to_value(row)?)
    }
}
