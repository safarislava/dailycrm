use crate::model::stage::{DetailedStage, Stage};
use chrono::{DateTime, Local};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct StageRow {
    project_id: Uuid,
    position: i32,
    title: String,
    description: Option<String>,
    deadline: Option<DateTime<Local>>,
    cost: Option<i32>,
}

#[derive(Clone)]
pub struct Stages {
    pool: PgPool,
}

impl Stages {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn stages(&self, project_id: Uuid) -> Result<Vec<Stage>, sqlx::Error> {
        let rows = sqlx::query_as::<_, StageRow>(
            "SELECT * FROM stages WHERE project_id = $1 ORDER BY position",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Stage::new(row.project_id, row.position, row.title))
            .collect())
    }

    pub async fn register(
        &self,
        project_id: Uuid,
        position: i32,
        title: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO stages(project_id, position, title) VALUES ($1, $2, $3)")
            .bind(project_id)
            .bind(position)
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn detailed_stage(
        &self,
        project_id: Uuid,
        position: i32,
    ) -> Result<DetailedStage, sqlx::Error> {
        let row = sqlx::query_as::<_, StageRow>(
            "SELECT * FROM stages WHERE project_id = $1 AND position = $2",
        )
        .bind(project_id)
        .bind(position)
        .fetch_one(&self.pool)
        .await?;
        let base = Stage::new(row.project_id, row.position, row.title);
        Ok(DetailedStage::new(base, row.description, row.deadline, row.cost))
    }

    pub async fn remove(&self, project_id: Uuid, position: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM stages WHERE project_id = $1 AND position = $2")
            .bind(project_id)
            .bind(position)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
