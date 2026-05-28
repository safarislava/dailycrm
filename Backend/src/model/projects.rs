use crate::contract::Projects;
use crate::model::project::{Project, ProjectDetails};
use crate::storage::Storage;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgProjects {
    pool: PgPool,
    storage: Storage,
}

impl PgProjects {
    pub fn new(pool: PgPool, storage: Storage) -> Self {
        Self { pool, storage }
    }
}

#[async_trait]
impl Projects for PgProjects {
    fn project(&self, id: Uuid) -> Project {
        Project::new(self.pool.clone(), self.storage.clone(), id)
    }

    async fn list(&self) -> Result<Vec<ProjectDetails>, sqlx::Error> {
        let ids = sqlx::query_scalar::<_, Uuid>("SELECT id FROM projects ORDER BY updated_at DESC")
            .fetch_all(&self.pool)
            .await?;

        Ok(ids
            .into_iter()
            .map(|id| ProjectDetails::new(self.pool.clone(), id))
            .collect())
    }

    async fn register(&self, title: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO projects (title) VALUES ($1)")
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
