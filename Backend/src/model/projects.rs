use crate::model::project::Project;
use crate::model::project_link::ProjectLink;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct Projects {
    pool: PgPool,
}

impl Projects {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Project>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (Uuid, String, DateTime<Utc>)>(
            "SELECT id, title, updated_at FROM projects ORDER BY updated_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|(id, title, updated_at)| Project::new(id, title, updated_at, self.pool.clone()))
            .collect())
    }

    pub async fn register(&self, title: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO projects (title) VALUES ($1)")
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub fn project_link(&self, id: Uuid) -> ProjectLink {
        ProjectLink::new(id, self.pool.clone())
    }

    pub async fn project_by_id(&self, id: Uuid) -> Result<Project, sqlx::Error> {
        let (id, title, updated_at) = sqlx::query_as::<_, (Uuid, String, DateTime<Utc>)>(
            "SELECT id, title, updated_at FROM projects WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(Project::new(id, title, updated_at, self.pool.clone()))
    }
}
