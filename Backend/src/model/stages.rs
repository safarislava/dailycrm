use crate::model::stage::{DetailedStage, Stage};
use crate::model::stage_link::StageLink;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct StageRow {
    project_id: Uuid,
    position: i32,
    title: String,
    description: Option<String>,
    deadline: Option<DateTime<Utc>>,
    cost: Option<i32>,
    completed: bool,
}

pub struct Stages {
    project_id: Uuid,
}

impl Stages {
    pub fn new(project_id: Uuid) -> Self {
        Self { project_id }
    }

    pub fn stage_link(&self, position: i32) -> StageLink {
        StageLink::new(self.project_id, position)
    }

    pub async fn list(&self, pool: &PgPool) -> Result<Vec<Stage>, sqlx::Error> {
        let rows = sqlx::query_as::<_, StageRow>(
            "SELECT * FROM stages WHERE project_id = $1 ORDER BY position",
        )
        .bind(self.project_id)
        .fetch_all(pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| Stage::new(r.project_id, r.position, r.title, r.deadline, r.completed))
            .collect())
    }

    pub async fn append(&self, title: String, pool: &PgPool) -> Result<(), sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row { max: Option<i32> }
        let row: Row = sqlx::query_as("SELECT MAX(position) AS max FROM stages WHERE project_id = $1")
            .bind(self.project_id)
            .fetch_one(pool)
            .await?;
        let position = row.max.unwrap_or(0) + 1;
        sqlx::query("INSERT INTO stages(project_id, position, title) VALUES ($1, $2, $3)")
            .bind(self.project_id)
            .bind(position)
            .bind(title)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn register(&self, position: i32, title: String, pool: &PgPool) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        sqlx::query(
            "UPDATE stages SET position = position + 1 WHERE project_id = $1 AND position >= $2",
        )
        .bind(self.project_id)
        .bind(position)
        .execute(&mut *tx)
        .await?;
        sqlx::query("INSERT INTO stages(project_id, position, title) VALUES ($1, $2, $3)")
            .bind(self.project_id)
            .bind(position)
            .bind(title)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn detailed_stage(&self, position: i32, pool: &PgPool) -> Result<DetailedStage, sqlx::Error> {
        let row = sqlx::query_as::<_, StageRow>(
            "SELECT * FROM stages WHERE project_id = $1 AND position = $2",
        )
        .bind(self.project_id)
        .bind(position)
        .fetch_one(pool)
        .await?;
        let base = Stage::new(row.project_id, row.position, row.title, row.deadline, row.completed);
        Ok(DetailedStage::new(base, row.description, row.cost))
    }
}