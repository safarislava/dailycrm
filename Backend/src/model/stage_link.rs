use crate::model::attachments::Attachments;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct StageLink {
    project_id: Uuid,
    position: i32,
    pool: PgPool,
}

impl StageLink {
    pub fn new(project_id: Uuid, position: i32, pool: PgPool) -> Self {
        Self {
            project_id,
            position,
            pool,
        }
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

    pub async fn update_title(&self, title: String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET title = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_deadline(
        &self,
        deadline: Option<DateTime<Utc>>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET deadline = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(deadline)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_description(
        &self,
        description: Option<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET description = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(description)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_cost(&self, cost: Option<i32>) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET cost = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(cost)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_completed(&self, completed: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET completed = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(completed)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}