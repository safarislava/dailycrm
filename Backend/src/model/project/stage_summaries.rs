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
            "SELECT project_id, parent_position, position, title, deadline,
                    (gip_confirmed AND payment_confirmed AND EXISTS(
                        SELECT 1 FROM attachments a
                        WHERE a.project_id = stages.project_id
                        AND a.parent_position = stages.parent_position
                        AND a.stage_position = stages.position AND a.is_act = TRUE
                    )) AS completed
             FROM stages WHERE project_id = $1 ORDER BY parent_position, position",
        )
        .bind(self.project.id())
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter()
            .map(|r| serde_json::to_value(r).map_err(|e| sqlx::Error::Decode(e.into())))
            .collect()
    }
}