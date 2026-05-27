use crate::contract::Projects;
use crate::model::project::Project;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgProjects {
    pool: PgPool,
}

impl PgProjects {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Projects for PgProjects {
    async fn list(&self) -> Result<Vec<Project>, sqlx::Error> {
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

    async fn register(&self, title: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO projects (title) VALUES ($1)")
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn rename(&self, id: Uuid, title: &str) -> Result<(), sqlx::Error> {
        let result = sqlx::query("UPDATE projects SET title = $2 WHERE id = $1")
            .bind(id)
            .bind(title)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    async fn remove(&self, id: Uuid) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }
}
