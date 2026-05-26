use crate::model::stages::Stages;
use sqlx::PgPool;
use uuid::Uuid;

pub struct ProjectLink {
    id: Uuid,
}

impl ProjectLink {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn stages(&self) -> Stages {
        Stages::new(self.id)
    }

    pub async fn rename(&self, title: &str, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE projects SET title = $2 WHERE id = $1")
            .bind(self.id)
            .bind(title)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn remove(self, pool: &PgPool) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(self.id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }
}
