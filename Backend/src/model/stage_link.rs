use crate::model::attachments::Attachments;
use sqlx::PgPool;
use uuid::Uuid;

pub struct StageLink {
    project_id: Uuid,
    position: i32,
    pool: PgPool,
}

impl StageLink {
    pub fn new(project_id: Uuid, position: i32, pool: PgPool) -> Self {
        Self { project_id, position, pool }
    }

    pub fn attachments(&self) -> Attachments {
        Attachments::new(self.project_id, self.position, self.pool.clone())
    }

    pub async fn remove(self) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM stages WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }
}