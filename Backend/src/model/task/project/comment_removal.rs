use crate::common::BoxError;
use crate::model::project::comment::Comment;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct CommentRemoval {
    pool: Arc<PgPool>,
    comment: Comment,
}

impl CommentRemoval {
    pub fn new(pool: Arc<PgPool>, comment: Comment) -> Self {
        Self { pool, comment }
    }
}

#[async_trait::async_trait]
impl Task for CommentRemoval {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let rows_affected = sqlx::query("DELETE FROM stage_comments WHERE id = $1")
            .bind(self.comment.id())
            .execute(self.pool.as_ref())
            .await?
            .rows_affected();
        if rows_affected == 0 {
            return Err("Comment not found".into());
        }
        Ok(())
    }
}