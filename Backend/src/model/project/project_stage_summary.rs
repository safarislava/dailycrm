use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::stage::Stage;

pub struct ProjectStageSummary {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl ProjectStageSummary {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl Contentable for ProjectStageSummary {
    type Output = serde_json::Value;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            project_id: Uuid,
            position: i32,
            title: String,
            deadline: Option<DateTime<Utc>>,
            completed: bool,
            project_title: String,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT s.project_id, s.position, s.title, s.deadline,
                    (s.gip_confirmed AND s.payment_confirmed AND EXISTS(
                        SELECT 1 FROM attachments a
                        WHERE a.project_id = s.project_id
                        AND a.stage_position = s.position AND a.is_act = TRUE
                    )) AS completed,
                    p.title AS project_title
             FROM stages s
             JOIN projects p ON p.id = s.project_id
             WHERE s.project_id = $1 AND s.position = $2",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.position())
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(serde_json::json!({
            "stage": {
                "project_id": row.project_id,
                "position": row.position,
                "title": row.title,
                "deadline": row.deadline,
                "completed": row.completed,
            },
            "project_title": row.project_title,
        }))
    }
}
