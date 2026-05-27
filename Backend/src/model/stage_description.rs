use sqlx::PgPool;
use uuid::Uuid;

pub struct StageDescription {
    project_id: Uuid,
    position: i32,
    description: Option<String>,
    pool: PgPool,
}

impl StageDescription {
    pub fn new(
        project_id: Uuid,
        position: i32,
        description: Option<String>,
        pool: PgPool,
    ) -> Self {
        Self { project_id, position, description, pool }
    }

    pub async fn save(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET description = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(&self.description)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}