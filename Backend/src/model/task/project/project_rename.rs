use crate::common::BoxError;
use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct ProjectRename {
    pool: Arc<PgPool>,
    project: Project,
    title: String,
}

impl ProjectRename {
    pub fn new(pool: Arc<PgPool>, project: Project, title: String) -> Self {
        Self {
            pool,
            project,
            title,
        }
    }
}

#[async_trait::async_trait]
impl Task for ProjectRename {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let result = sqlx::query("UPDATE projects SET title = $2 WHERE id = $1")
            .bind(self.project.id())
            .bind(&self.title)
            .execute(self.pool.as_ref())
            .await?;
        if result.rows_affected() == 0 {
            return Err(BoxError::from(sqlx::Error::RowNotFound));
        }
        Ok(())
    }
}
