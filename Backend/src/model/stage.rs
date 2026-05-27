use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::contract::{Attachments, Details};
use crate::model::attachments::PgAttachments;
use crate::storage::Storage;

pub struct Stage {
    pool: PgPool,
    storage: Storage,
    project_id: Uuid,
    position: i32,
}

impl Stage {
    pub fn new(pool: PgPool, storage: Storage, project_id: Uuid, position: i32) -> Self {
        Stage {
            pool,
            storage,
            project_id,
            position,
        }
    }

    pub fn attachments(&self) -> Box<dyn Attachments> {
        Box::new(PgAttachments::new(
            self.pool.clone(),
            self.storage.clone(),
            self.project_id,
            self.position,
        ))
    }
}

#[async_trait]
impl Details for Stage {
    type Detail = StageDetails;

    async fn details(&self) -> Result<StageDetails, sqlx::Error> {
        Ok(StageDetails::new(self.pool.clone(), self.project_id, self.position))
    }
}

pub struct StageDetails {
    pool: PgPool,
    project_id: Uuid,
    position: i32,
}

impl StageDetails {
    pub fn new(pool: PgPool, project_id: Uuid, position: i32) -> Self {
        StageDetails {
            pool,
            project_id,
            position,
        }
    }

    pub async fn data(&self) -> Result<impl Serialize, sqlx::Error> {
        #[derive(sqlx::FromRow, Serialize)]
        struct Row {
            project_id: Uuid,
            position: i32,
            title: String,
            deadline: Option<DateTime<Utc>>,
            completed: bool,
            description: Option<String>,
            cost: Option<i32>,
        }
        sqlx::query_as::<_, Row>(
            "SELECT project_id, position, title, deadline, completed, description, cost
             FROM stages WHERE project_id = $1 AND position = $2",
        )
        .bind(self.project_id)
        .bind(self.position)
        .fetch_one(&self.pool)
        .await
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

#[derive(Serialize)]
pub struct StageSummary {
    project_id: Uuid,
    position: i32,
    title: String,
    deadline: Option<DateTime<Utc>>,
    completed: bool,
}

impl StageSummary {
    pub fn new(
        project_id: Uuid,
        position: i32,
        title: String,
        deadline: Option<DateTime<Utc>>,
        completed: bool,
    ) -> Self {
        StageSummary {
            project_id,
            position,
            title,
            deadline,
            completed,
        }
    }
}

#[derive(Serialize)]
pub struct StageSummaryWithProjectTitle {
    stage: StageSummary,
    project_title: String,
}

impl StageSummaryWithProjectTitle {
    pub fn new(stage: StageSummary, project_title: String) -> Self {
        StageSummaryWithProjectTitle {
            stage,
            project_title,
        }
    }
}