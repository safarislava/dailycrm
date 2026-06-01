use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::comment::Comment;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use serde::Serialize;
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
impl Contentable for DetailedComment {
    type Output = serde_json::Value;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow, Serialize)]
        struct Row {
            id: Uuid,
            text: String,
            author: String,
            created_at: DateTime<Utc>,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT c.id, c.text, u.username AS author, c.created_at \
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