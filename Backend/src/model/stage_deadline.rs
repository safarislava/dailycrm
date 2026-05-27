use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct StageDeadline {
    project_id: Uuid,
    position: i32,
    deadline: Option<DateTime<Utc>>,
    pool: PgPool,
}

impl StageDeadline {
    pub fn new(
        project_id: Uuid,
        position: i32,
        deadline: Option<DateTime<Utc>>,
        pool: PgPool,
    ) -> Self {
        Self { project_id, position, deadline, pool }
    }

    pub async fn save(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET deadline = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(self.deadline)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}