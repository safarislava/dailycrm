use crate::model::notification::burning_deadline::BurningDeadline;
use crate::model::project::contract::list::List;
use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use std::sync::Arc;

pub struct BurningDeadlines {
    pool: Arc<PgPool>,
}

impl BurningDeadlines {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl List for BurningDeadlines {
    type Output = BurningDeadline;

    async fn items(&self) -> Result<Vec<BurningDeadline>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            project_title: String,
            stage_title: String,
            deadline: DateTime<Utc>,
        }
        let tomorrow = (Utc::now() + Duration::days(1)).date_naive();
        let start = tomorrow
            .and_hms_opt(0, 0, 0)
            .expect("valid midnight")
            .and_utc();
        let end = start + Duration::days(1);
        let rows = sqlx::query_as::<_, Row>(
            "SELECT p.title AS project_title, s.title AS stage_title, s.deadline
             FROM stages s
             JOIN projects p ON p.id = s.project_id
             WHERE s.completed = FALSE
               AND s.deadline >= $1 AND s.deadline < $2
             ORDER BY s.deadline",
        )
        .bind(start)
        .bind(end)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| BurningDeadline::new(r.project_title, r.stage_title, r.deadline))
            .collect())
    }
}
