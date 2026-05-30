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
            description: Option<String>,
            cost: Option<i32>,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT project_id, position, title, deadline, completed, description, cost
             FROM stages WHERE project_id = $1 AND position = $2",
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
            "description": row.description,
            "cost": row.cost,
        }))
    }
}
