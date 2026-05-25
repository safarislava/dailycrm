use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Project {
    id: Uuid,
    title: String,
    updated_at: DateTime<Utc>,
    #[serde(skip)]
    pool: PgPool,
}

impl Project {
    pub fn new(id: Uuid, title: String, updated_at: DateTime<Utc>, pool: PgPool) -> Self {
        Project {
            id,
            title,
            updated_at,
            pool,
        }
    }

    pub async fn rename(&self, title: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE projects SET title = $2 WHERE id = $1")
            .bind(self.id)
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn remove(&self) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(self.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
