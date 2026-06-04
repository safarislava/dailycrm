use crate::model::project::contract::list::List;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

pub struct Stages {
    pool: Arc<PgPool>,
    project: Project,
}

impl Stages {
    pub fn new(pool: Arc<PgPool>, project: Project) -> Self {
        Self { pool, project }
    }
}

#[async_trait]
impl List for Stages {
    type Output = Stage;

    async fn items(&self) -> Result<Vec<Stage>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct StageRow {
            parent_position: i32,
            position: i32,
        }
        let rows = sqlx::query_as::<_, StageRow>(
            "SELECT parent_position, position FROM stages WHERE project_id = $1 ORDER BY parent_position, position",
        )
        .bind(self.project.id())
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| Stage::new_substage(self.project.clone(), r.parent_position, r.position))
            .collect())
    }
}