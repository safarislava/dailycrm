use sqlx::PgPool;
use uuid::Uuid;

pub struct StageCompleted {
    project_id: Uuid,
    position: i32,
    completed: bool,
    pool: PgPool,
}

impl StageCompleted {
    pub fn new(project_id: Uuid, position: i32, completed: bool, pool: PgPool) -> Self {
        Self { project_id, position, completed, pool }
    }

    pub async fn save(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET completed = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(self.completed)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}