use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct CommentCreation {
    pool: Arc<PgPool>,
    stage: Stage,
    author: User,
    text: String,
}

impl CommentCreation {
    pub fn new(pool: Arc<PgPool>, stage: Stage, author: User, text: String) -> Self {
        Self { pool, stage, author, text }
    }
}

#[async_trait::async_trait]
impl Task for CommentCreation {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query(
            "INSERT INTO stage_comments(project_id, stage_position, author_id, text) \
             VALUES ($1, $2, $3, $4)",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.position())
        .bind(self.author.id())
        .bind(&self.text)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}