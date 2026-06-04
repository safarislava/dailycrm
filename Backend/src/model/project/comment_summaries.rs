use crate::model::project::contract::list::List;
use crate::model::project::stage::Stage;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct CommentSummaries {
    pool: Arc<PgPool>,
    stage: Stage,
    before: Option<Uuid>,
}

impl CommentSummaries {
    pub fn new(pool: Arc<PgPool>, stage: Stage, before: Option<Uuid>) -> Self {
        Self { pool, stage, before }
    }
}

#[async_trait]
impl List for CommentSummaries {
    type Output = serde_json::Value;

    async fn items(&self) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        #[derive(sqlx::FromRow, Serialize)]
        struct Row {
            id: Uuid,
            text: String,
            author: String,
            is_system: bool,
            created_at: DateTime<Utc>,
        }
        let mut rows = match self.before {
            None => {
                sqlx::query_as::<_, Row>(
                    "SELECT c.id, c.text, u.username AS author, c.is_system, c.created_at \
                     FROM stage_comments c \
                     JOIN users u ON u.id = c.author_id \
                     WHERE c.project_id = $1 AND c.parent_position = $2 AND c.stage_position = $3 \
                     ORDER BY c.created_at DESC, c.id DESC \
                     LIMIT 25",
                )
                .bind(self.stage.project().id())
                .bind(self.stage.parent_position())
                .bind(self.stage.position())
                .fetch_all(self.pool.as_ref())
                .await?
            }
            Some(before) => {
                sqlx::query_as::<_, Row>(
                    "SELECT c.id, c.text, u.username AS author, c.is_system, c.created_at \
                     FROM stage_comments c \
                     JOIN users u ON u.id = c.author_id \
                     WHERE c.project_id = $1 AND c.parent_position = $2 AND c.stage_position = $3 \
                     AND (c.created_at, c.id) < \
                         (SELECT created_at, id FROM stage_comments WHERE id = $4) \
                     ORDER BY c.created_at DESC, c.id DESC \
                     LIMIT 25",
                )
                .bind(self.stage.project().id())
                .bind(self.stage.parent_position())
                .bind(self.stage.position())
                .bind(before)
                .fetch_all(self.pool.as_ref())
                .await?
            }
        };
        rows.reverse();
        rows.into_iter()
            .map(|r| serde_json::to_value(r).map_err(|e| sqlx::Error::Decode(e.into())))
            .collect()
    }
}