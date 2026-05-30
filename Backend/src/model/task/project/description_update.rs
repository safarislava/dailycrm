use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct DescriptionUpdate {
    pool: Arc<PgPool>,
    stage: Stage,
    description: Option<String>,
}

impl DescriptionUpdate {
    pub fn new(pool: Arc<PgPool>, stage: Stage, description: Option<String>) -> Self {
        Self {
            pool,
            stage,
            description,
        }
    }
}

#[async_trait::async_trait]
impl Task for DescriptionUpdate {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("UPDATE stages SET description = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.stage.project().id())
            .bind(self.stage.position())
            .bind(&self.description)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
