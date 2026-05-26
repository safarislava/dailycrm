use sqlx::PgPool;
use uuid::Uuid;

pub struct ProjectLink {
    id: Uuid,
    pool: PgPool,
}

impl ProjectLink {
    pub fn new(id: Uuid, pool: PgPool) -> Self {
        Self { id, pool }
    }

    pub async fn remove(self) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(self.id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }
}