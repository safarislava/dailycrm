use crate::common::BoxError;
use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct ProjectRemoval {
    pool: Arc<PgPool>,
    project: Project,
}

impl ProjectRemoval {
    pub fn new(pool: Arc<PgPool>, project: Project) -> Self {
        Self { pool, project }
    }
}

#[async_trait::async_trait]
impl Task for ProjectRemoval {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let result = sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(self.project.id())
            .execute(self.pool.as_ref())
            .await?;
        if result.rows_affected() == 0 {
            return Err(BoxError::from(sqlx::Error::RowNotFound));
        }
        Ok(())
    }
}
