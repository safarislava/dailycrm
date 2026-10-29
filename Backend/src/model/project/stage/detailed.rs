use crate::model::contract::box_error::BoxError;
use crate::model::project::contract::json::Json;
use crate::model::project::stage::cost::payment::StagePayment;
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
        struct DetailedStageJson {
            project_id: Uuid,
            parent_position: i32,
            position: i32,
            title: String,
            deadline: Option<DateTime<Utc>>,
            completed: bool,
            #[sqlx(json)]
            payments: Vec<StagePayment>,
            gip_confirmed: bool,
        }

        let stage = sqlx::query_as::<_, DetailedStageJson>(
            "SELECT s.project_id, s.parent_position, s.position, s.title, s.deadline, s.completed, s.gip_confirmed,
                    COALESCE(
                        (
                            SELECT json_agg(
                                json_build_object(
                                    'kind', p.kind,
                                    'cost', p.cost,
                                    'confirmed', p.confirmed
                                )
                            )
                            FROM stage_payments p
                            WHERE p.project_id = s.project_id
                              AND p.parent_position = s.parent_position
                              AND p.stage_position = s.position
                        ),
                        '[]'::json
                    ) AS payments
             FROM detailed_stages s
             WHERE s.project_id = $1 AND s.parent_position = $2 AND s.position = $3",
        )
        .bind(self.stage_id.project_id().id())
        .bind(self.stage_id.parent_position())
        .bind(self.stage_id.position())
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(serde_json::to_value(stage)?)
    }
}
