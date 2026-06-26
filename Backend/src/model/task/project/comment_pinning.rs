use crate::common::BoxError;
use crate::model::project::comment::Comment;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct CommentPinning {
    pool: Arc<PgPool>,
    comment: Comment,
    pinned: bool,
}

impl CommentPinning {
    pub fn new(pool: Arc<PgPool>, comment: Comment, pinned: bool) -> Self {
        Self {
            pool,
            comment,
            pinned,
        }
    }
}

#[async_trait::async_trait]
impl Task for CommentPinning {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let rows_affected = sqlx::query("UPDATE stage_comments SET is_pinned = $1 WHERE id = $2")
            .bind(self.pinned)
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
