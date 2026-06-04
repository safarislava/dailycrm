use crate::common::BoxError;
use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageAppending {
    pool: Arc<PgPool>,
    project: Project,
    parent_position: i32,
    title: String,
}

impl StageAppending {
    pub fn new(pool: Arc<PgPool>, project: Project, title: String) -> Self {
        Self { pool, project, parent_position: 0, title }
    }

    pub fn sub(pool: Arc<PgPool>, project: Project, parent_position: i32, title: String) -> Self {
        Self { pool, project, parent_position, title }
    }
}

#[async_trait::async_trait]
impl Task for StageAppending {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            max: Option<i32>,
        }
        let row: Row = sqlx::query_as(
            "SELECT MAX(position) AS max FROM stages WHERE project_id = $1 AND parent_position = $2",
        )
        .bind(self.project.id())
        .bind(self.parent_position)
        .fetch_one(self.pool.as_ref())
        .await?;
        let position = row.max.unwrap_or(0) + 1;
        sqlx::query(
            "INSERT INTO stages(project_id, parent_position, position, title) VALUES ($1, $2, $3, $4)",
        )
        .bind(self.project.id())
        .bind(self.parent_position)
        .bind(position)
        .bind(&self.title)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}