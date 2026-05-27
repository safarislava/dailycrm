use sqlx::PgPool;
use uuid::Uuid;

pub struct PositionedStage {
    project_id: Uuid,
    position: i32,
    title: String,
    pool: PgPool,
}

impl PositionedStage {
    pub fn new(project_id: Uuid, position: i32, title: String, pool: PgPool) -> Self {
        Self { project_id, position, title, pool }
    }

    pub async fn save(&self) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query(
            "UPDATE stages SET position = position + 1 WHERE project_id = $1 AND position >= $2",
        )
        .bind(self.project_id)
        .bind(self.position)
        .execute(&mut *transaction)
        .await?;
        sqlx::query("INSERT INTO stages(project_id, position, title) VALUES ($1, $2, $3)")
            .bind(self.project_id)
            .bind(self.position)
            .bind(&self.title)
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        Ok(())
    }
}