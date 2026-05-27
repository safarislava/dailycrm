use sqlx::PgPool;
use uuid::Uuid;

pub struct StageCost {
    project_id: Uuid,
    position: i32,
    cost: Option<i32>,
    pool: PgPool,
}

impl StageCost {
    pub fn new(project_id: Uuid, position: i32, cost: Option<i32>, pool: PgPool) -> Self {
        Self { project_id, position, cost, pool }
    }

    pub async fn save(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET cost = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(self.cost)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}