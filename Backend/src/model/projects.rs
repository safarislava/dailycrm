use crate::model::project::Project;
use crate::model::project_link::ProjectLink;
use crate::model::stage::{Stage, StageWithProjectTitle};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct Projects {
    pool: PgPool,
}

impl Projects {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn project_link(&self, id: Uuid) -> ProjectLink {
        ProjectLink::new(id, self.pool.clone())
    }

    pub async fn list(&self) -> Result<Vec<Project>, sqlx::Error> {
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

    pub async fn deadlines(&self) -> Result<Vec<StageWithProjectTitle>, sqlx::Error> {
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