use crate::contract::Deadlines;
use crate::model::stage::{StageSummary, StageSummaryWithProjectTitle};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgDeadlines {
    pool: PgPool,
}

impl PgDeadlines {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Deadlines for PgDeadlines {
    async fn list(&self) -> Result<Vec<StageSummaryWithProjectTitle>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            project_id: Uuid,
            project_title: String,
            position: i32,
            stage_title: String,
            deadline: DateTime<Utc>,
            completed: bool,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT s.project_id, p.title AS project_title, s.position,
                    s.title AS stage_title, s.deadline, s.completed
             FROM stages s
             JOIN projects p ON p.id = s.project_id
             WHERE s.deadline IS NOT NULL
             ORDER BY s.deadline",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                StageSummaryWithProjectTitle::new(
                    StageSummary::new(
                        r.project_id,
                        r.position,
                        r.stage_title,
                        Some(r.deadline),
                        r.completed,
                    ),
                    r.project_title,
                )
            })
            .collect())
    }
}
