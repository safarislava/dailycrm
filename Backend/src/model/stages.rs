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
    pool: PgPool,
}

impl Stages {
    pub fn new(project_id: Uuid, pool: PgPool) -> Self {
        Self { project_id, pool }
    }

    pub fn stage_link(&self, position: i32) -> StageLink {
        StageLink::new(self.project_id, position, self.pool.clone())
    }

    pub async fn list(&self) -> Result<Vec<Stage>, sqlx::Error> {
        let rows = sqlx::query_as::<_, StageRow>(
            "SELECT * FROM stages WHERE project_id = $1 ORDER BY position",
        )
        .bind(self.project_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| Stage::new(r.project_id, r.position, r.title, r.deadline, r.completed))
            .collect())
    }

    pub async fn detailed_stage(&self, position: i32) -> Result<DetailedStage, sqlx::Error> {
        let row = sqlx::query_as::<_, StageRow>(
            "SELECT * FROM stages WHERE project_id = $1 AND position = $2",
        )
        .bind(self.project_id)
        .bind(position)
        .fetch_one(&self.pool)
        .await?;
        let base = Stage::new(
            row.project_id,
            row.position,
            row.title,
            row.deadline,
            row.completed,
        );
        Ok(DetailedStage::new(base, row.description, row.cost))
    }
}