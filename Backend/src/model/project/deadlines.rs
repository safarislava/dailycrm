use crate::model::project::contract::list::List;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct Deadlines {
    pool: Arc<PgPool>,
}

impl Deadlines {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl List for Deadlines {
    type Output = serde_json::Value;

    async fn items(&self) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            project_id: Uuid,
            parent_position: i32,
            position: i32,
            title: String,
            deadline: Option<DateTime<Utc>>,
            completed: bool,
            project_title: String,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT s.project_id, s.parent_position, s.position, s.title, s.deadline,
                    (s.gip_confirmed AND s.payment_confirmed AND EXISTS(
                        SELECT 1 FROM attachments a
                        WHERE a.project_id = s.project_id
                        AND a.parent_position = s.parent_position
                        AND a.stage_position = s.position AND a.is_act = TRUE
                    )) AS completed,
                    p.title AS project_title
             FROM stages s
             JOIN projects p ON p.id = s.project_id
             WHERE s.deadline IS NOT NULL
             ORDER BY s.deadline",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| {
                serde_json::json!({
                    "stage": {
                        "project_id": r.project_id,
                        "parent_position": r.parent_position,
                        "position": r.position,
                        "title": r.title,
                        "deadline": r.deadline,
                        "completed": r.completed,
                    },
                    "project_title": r.project_title,
                })
            })
            .collect())
    }
}