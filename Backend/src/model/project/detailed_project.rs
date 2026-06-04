use crate::common::BoxError;
use crate::model::project::contract::json::Json;
use crate::model::project::project::Project;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct DetailedProject {
    pool: Arc<PgPool>,
    project: Project,
}

impl DetailedProject {
    pub fn new(pool: Arc<PgPool>, project: Project) -> Self {
        DetailedProject { pool, project }
    }
}

#[async_trait::async_trait]
impl Json for DetailedProject {

    async fn json(&self) -> Result<serde_json::Value, BoxError> {
        #[derive(sqlx::FromRow, Serialize)]
        struct Row {
            id: Uuid,
            title: String,
            updated_at: DateTime<Utc>,
        }
        let row =
            sqlx::query_as::<_, Row>("SELECT id, title, updated_at FROM projects WHERE id = $1")
                .bind(self.project.id())
                .fetch_one(self.pool.as_ref())
                .await?;
        Ok(serde_json::to_value(row)?)
    }
}
