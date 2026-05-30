use crate::model::project::contract::list::List;
use crate::model::project::project::Project;
use crate::model::project::project_stage_summary::ProjectStageSummary;
use crate::model::project::stage::Stage;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct Deadlines {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
}

impl Deadlines {
    pub fn new(pool: Arc<PgPool>, storage: Arc<Storage>) -> Self {
        Self { pool, storage }
    }
}

#[async_trait::async_trait]
impl List for Deadlines {
    type Output = ProjectStageSummary;

    async fn items(&self) -> Result<Vec<ProjectStageSummary>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            project_id: Uuid,
            position: i32,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT project_id, position FROM stages
             WHERE deadline IS NOT NULL ORDER BY deadline",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| {
                let project = Project::new(r.project_id);
                let stage = Stage::new(project, r.position);
                ProjectStageSummary::new(self.pool.clone(), stage)
            })
            .collect())
    }
}
