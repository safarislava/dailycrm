use crate::contract::Stages;
use crate::model::stage::{Stage, StageSummary};
use crate::storage::Storage;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct StageSummaryRow {
    project_id: Uuid,
    position: i32,
    title: String,
    deadline: Option<DateTime<Utc>>,
    completed: bool,
}

pub struct PgStages {
    pool: PgPool,
    storage: Storage,
    project_id: Uuid,
}

impl PgStages {
    pub fn new(pool: PgPool, storage: Storage, project_id: Uuid) -> Self {
        Self {
            pool,
            storage,
            project_id,
        }
    }
}

#[async_trait]
impl Stages for PgStages {
    fn stage(&self, position: i32) -> Stage {
        Stage::new(
            self.pool.clone(),
            self.storage.clone(),
            self.project_id,
            position,
        )
    }

    async fn list(&self) -> Result<Vec<StageSummary>, sqlx::Error> {
        let rows = sqlx::query_as::<_, StageSummaryRow>(
            "SELECT project_id, position, title, deadline, completed
             FROM stages WHERE project_id = $1 ORDER BY position",
        )
        .bind(self.project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| StageSummary::new(r.project_id, r.position, r.title, r.deadline, r.completed))
            .collect())
    }

    async fn append(&self, title: String) -> Result<(), sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            max: Option<i32>,
        }
        let row: Row =
            sqlx::query_as("SELECT MAX(position) AS max FROM stages WHERE project_id = $1")
                .bind(self.project_id)
                .fetch_one(&self.pool)
                .await?;
        let position = row.max.unwrap_or(0) + 1;
        sqlx::query("INSERT INTO stages(project_id, position, title) VALUES ($1, $2, $3)")
            .bind(self.project_id)
            .bind(position)
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn insert(&self, position: i32, title: String) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query(
            "UPDATE stages SET position = position + 1
             WHERE project_id = $1 AND position >= $2",
        )
        .bind(self.project_id)
        .bind(position)
        .execute(&mut *transaction)
        .await?;
        sqlx::query("INSERT INTO stages(project_id, position, title) VALUES ($1, $2, $3)")
            .bind(self.project_id)
            .bind(position)
            .bind(title)
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn remove(&self, position: i32) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM stages WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(position)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }
}