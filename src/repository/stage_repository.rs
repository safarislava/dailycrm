use chrono::{DateTime, Local};
use sqlx::PgPool;
use uuid::Uuid;

pub type StageRow = (Uuid, Uuid, i64, String, String, DateTime<Local>, i64);

#[derive(Clone)]
pub struct StageRepository {
    pool: PgPool,
}

impl StageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_project_id(&self, project_id: Uuid) -> Result<Vec<StageRow>, sqlx::Error> {
        sqlx::query_as::<_, StageRow>(
            "SELECT * FROM stages WHERE project_id = $1 ORDER BY position",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create(
        &self,
        project_id: Uuid,
        position: i64,
        title: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO stages(project_id, position, title) VALUES ($1, $2, $3)")
            .bind(project_id)
            .bind(position)
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn find_by_id(
        &self,
        project_id: Uuid,
        stage_id: Uuid,
    ) -> Result<StageRow, sqlx::Error> {
        sqlx::query_as::<_, StageRow>(
            "SELECT * FROM stages WHERE project_id = $1 AND stage_id = $2",
        )
        .bind(project_id)
        .bind(stage_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, project_id: Uuid, stage_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM stages WHERE project_id = $1 AND stage_id = $2")
            .bind(project_id)
            .bind(stage_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
