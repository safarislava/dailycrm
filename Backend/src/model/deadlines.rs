use crate::model::stage::{Stage, StageWithProjectTitle};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct Deadlines {
    pool: PgPool,
}

impl Deadlines {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<StageWithProjectTitle>, sqlx::Error> {
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
                StageWithProjectTitle::new(
                    Stage::new(
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