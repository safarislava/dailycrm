use crate::common::BoxError;
use crate::model::project::contract::json::Json;
use crate::model::project::comment::Comment;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct DetailedComment {
    pool: Arc<PgPool>,
    comment: Comment,
}

impl DetailedComment {
    pub fn new(pool: Arc<PgPool>, comment: Comment) -> Self {
        Self { pool, comment }
    }
}

#[async_trait::async_trait]
impl Json for DetailedComment {

    async fn json(&self) -> Result<serde_json::Value, BoxError> {
        #[derive(sqlx::FromRow, Serialize)]
        struct Row {
            id: Uuid,
            text: String,
            author: String,
            is_system: bool,
            created_at: DateTime<Utc>,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT c.id, c.text, u.username AS author, c.is_system, c.created_at \
             FROM stage_comments c \
             JOIN users u ON u.id = c.author_id \
             WHERE c.id = $1",
        )
        .bind(self.comment.id())
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(serde_json::to_value(row)?)
    }
}
