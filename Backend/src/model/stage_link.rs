use crate::model::attachments::Attachments;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct StageLink {
    project_id: Uuid,
    position: i32,
}

impl StageLink {
    pub fn new(project_id: Uuid, position: i32) -> Self {
        Self { project_id, position }
    }

    pub fn attachments(&self) -> Attachments {
        Attachments::new(self.project_id, self.position)
    }

    pub async fn remove(self, pool: &PgPool) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM stages WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    pub async fn update_title(&self, title: String, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET title = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(title)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_deadline(&self, deadline: Option<DateTime<Utc>>, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET deadline = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(deadline)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_description(&self, description: Option<String>, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET description = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(description)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_cost(&self, cost: Option<i32>, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET cost = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(cost)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_completed(&self, completed: bool, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET completed = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.project_id)
            .bind(self.position)
            .bind(completed)
            .execute(pool)
            .await?;
        Ok(())
    }
}