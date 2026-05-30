use crate::model::project::contract::list::List;
use crate::model::project::project::Project;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct Projects {
    pool: Arc<PgPool>,
}

impl Projects {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl List for Projects {
    type Output = Project;

    async fn items(&self) -> Result<Vec<Project>, sqlx::Error> {
        let ids = sqlx::query_scalar::<_, Uuid>("SELECT id FROM projects ORDER BY updated_at DESC")
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(ids.into_iter().map(|id| Project::new(id)).collect())
    }
}
