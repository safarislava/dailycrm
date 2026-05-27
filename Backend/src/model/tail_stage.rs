use sqlx::PgPool;
use uuid::Uuid;

pub struct TailStage {
    project_id: Uuid,
    title: String,
    pool: PgPool,
}

impl TailStage {
    pub fn new(project_id: Uuid, title: String, pool: PgPool) -> Self {
        Self { project_id, title, pool }
    }

    pub async fn save(&self) -> Result<(), sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            max: Option<i32>,
        }
        let row: Row =
            sqlx::query_as("SELECT MAX(position) AS max FROM stages WHERE project_id = $1")
                .bind(self.project_id)
                .fetch_one(&self.pool)
                .await?;
        let position = row.max.unwrap_or(0) + 1;
        sqlx::query("INSERT INTO stages(project_id, position, title) VALUES ($1, $2, $3)")
            .bind(self.project_id)
            .bind(position)
            .bind(&self.title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}