use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageRename {
    pool: Arc<PgPool>,
    stage: Stage,
    title: String,
}

impl StageRename {
    pub fn new(pool: Arc<PgPool>, stage: Stage, title: String) -> Self {
        Self { pool, stage, title }
    }
}

#[async_trait::async_trait]
impl Task for StageRename {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("UPDATE stages SET title = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.stage.project().id())
            .bind(self.stage.position())
            .bind(&self.title)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
