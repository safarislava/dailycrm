use sqlx::PgPool;
use uuid::Uuid;

pub struct StageTitle {
    project_id: Uuid,
    position: i32,
    title: String,
    pool: PgPool,
}

impl StageTitle {
    pub fn new(project_id: Uuid, position: i32, title: String, pool: PgPool) -> Self {
        Self { project_id, position, title, pool }
    }

    pub async fn save(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET title = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(&self.title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}