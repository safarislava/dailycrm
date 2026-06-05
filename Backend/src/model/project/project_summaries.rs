use crate::model::project::contract::list::List;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct ProjectSummaries {
    pool: Arc<PgPool>,
}

impl ProjectSummaries {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl List for ProjectSummaries {
    type Output = serde_json::Value;

    async fn items(&self) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        #[derive(sqlx::FromRow, Serialize)]
        struct Row {
            id: Uuid,
            title: String,
            updated_at: DateTime<Utc>,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT id, title, updated_at FROM projects ORDER BY updated_at DESC",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter()
            .map(|r| serde_json::to_value(r).map_err(|e| sqlx::Error::Decode(e.into())))
            .collect()
    }
}