use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::project::contract::details::Details;
use crate::model::project::contract::stages::Stages;
use crate::model::project::stages::PgStages;
use crate::storage::Storage;

pub struct Project {
    pool: PgPool,
    storage: Storage,
    id: Uuid,
}

impl Project {
    pub fn new(pool: PgPool, storage: Storage, id: Uuid) -> Self {
        Project { pool, storage, id }
    }

    pub fn stages(&self) -> Box<dyn Stages> {
        Box::new(PgStages::new(
            self.pool.clone(),
            self.storage.clone(),
            self.id,
        ))
    }

    pub async fn remove(&self) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(self.id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }
}

#[async_trait]
impl Details for Project {
    type Detail = ProjectDetails;

    async fn details(&self) -> Result<ProjectDetails, sqlx::Error> {
        Ok(ProjectDetails::new(self.pool.clone(), self.id))
    }
}

pub struct ProjectDetails {
    pool: PgPool,
    id: Uuid,
}

impl ProjectDetails {
    pub fn new(pool: PgPool, id: Uuid) -> Self {
        ProjectDetails { pool, id }
    }

    pub async fn data(&self) -> Result<impl Serialize, sqlx::Error> {
        #[derive(sqlx::FromRow, Serialize)]
        struct Row {
            id: Uuid,
            title: String,
            updated_at: DateTime<Utc>,
        }
        sqlx::query_as::<_, Row>("SELECT id, title, updated_at FROM projects WHERE id = $1")
            .bind(self.id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn rename(&self, title: &str) -> Result<(), sqlx::Error> {
        let result = sqlx::query("UPDATE projects SET title = $2 WHERE id = $1")
            .bind(self.id)
            .bind(title)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }
}
