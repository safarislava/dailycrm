use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageRemoval {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl StageRemoval {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl Task for StageRemoval {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let result = sqlx::query("DELETE FROM stages WHERE project_id = $1 AND position = $2")
            .bind(self.stage.project().id())
            .bind(self.stage.position())
            .execute(self.pool.as_ref())
            .await?;
        if result.rows_affected() == 0 {
            return Err(BoxError::from(sqlx::Error::RowNotFound));
        }
        Ok(())
    }
}
