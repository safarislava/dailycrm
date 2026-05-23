use crate::model::project::Project;
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

    pub async fn projects(&self) -> Result<Vec<Project>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (Uuid, String, DateTime<Utc>)>(
            "SELECT id, title, updated_at FROM projects ORDER BY updated_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|(id, title, updated_at)| Project::new(id, title, updated_at))
            .collect())
    }

    pub async fn register(&self, title: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO projects (title) VALUES ($1)")
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn rename(&self, id: Uuid, title: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE projects SET title = $2 WHERE id = $1")
            .bind(id)
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn remove(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
