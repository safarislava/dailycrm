use crate::model::project::contract::list::List;
use crate::model::project::project::Project;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct StageSummaries {
    pool: Arc<PgPool>,
    project: Project,
}

impl StageSummaries {
    pub fn new(pool: Arc<PgPool>, project: Project) -> Self {
        Self { pool, project }
    }
}

#[async_trait]
impl List for StageSummaries {
    type Output = serde_json::Value;

    async fn items(&self) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        #[derive(sqlx::FromRow, Serialize)]
        struct Row {
            project_id: Uuid,
            parent_position: i32,
            position: i32,
            title: String,
            deadline: Option<DateTime<Utc>>,
            completed: bool,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT s.project_id, s.parent_position, s.position, s.title, s.deadline,
                    (s.gip_confirmed AND s.payment_confirmed) AS completed
             FROM stages s
             LEFT JOIN LATERAL (
                 SELECT 1 AS id FROM attachments a
                 WHERE a.project_id = s.project_id
                 AND a.parent_position = s.parent_position
                 AND a.stage_position = s.position AND a.is_act = TRUE
                 LIMIT 1
             ) act ON TRUE
             WHERE s.project_id = $1 ORDER BY s.parent_position, s.position",
        )
        .bind(self.project.id())
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter()
            .map(|r| serde_json::to_value(r).map_err(|e| sqlx::Error::Decode(e.into())))
            .collect()
    }
}