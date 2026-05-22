use crate::model::stage::{DeadlineItem, DetailedStage, Stage};
use chrono::NaiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct StageRow {
    project_id: Uuid,
    position: i32,
    title: String,
    description: Option<String>,
    deadline: Option<NaiveDateTime>,
    cost: Option<i32>,
    completed: bool,
}

#[derive(Clone)]
pub struct Stages {
    pool: PgPool,
}

impl Stages {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn stages(&self, project_id: Uuid) -> Result<Vec<Stage>, sqlx::Error> {
        let rows = sqlx::query_as::<_, StageRow>(
            "SELECT * FROM stages WHERE project_id = $1 ORDER BY position",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Stage::new(row.project_id, row.position, row.title, row.deadline, row.completed))
            .collect())
    }

    pub async fn append(&self, project_id: Uuid, title: String) -> Result<(), sqlx::Error> {
        let max: Option<i32> =
            sqlx::query_scalar("SELECT MAX(position) FROM stages WHERE project_id = $1")
                .bind(project_id)
                .fetch_one(&self.pool)
                .await?;
        let position = max.unwrap_or(0) + 1;
        sqlx::query("INSERT INTO stages(project_id, position, title) VALUES ($1, $2, $3)")
            .bind(project_id)
            .bind(position)
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn register(
        &self,
        project_id: Uuid,
        position: i32,
        title: String,
    ) -> Result<(), sqlx::Error> {
        let mut pool = self.pool.begin().await?;
        sqlx::query(
            "UPDATE stages SET position = position + 1 WHERE project_id = $1 AND position >= $2",
        )
        .bind(project_id)
        .bind(position)
        .execute(&mut *pool)
        .await?;
        sqlx::query("INSERT INTO stages(project_id, position, title) VALUES ($1, $2, $3)")
            .bind(project_id)
            .bind(position)
            .bind(title)
            .execute(&mut *pool)
            .await?;
        pool.commit().await?;
        Ok(())
    }

    pub async fn detailed_stage(
        &self,
        project_id: Uuid,
        position: i32,
    ) -> Result<DetailedStage, sqlx::Error> {
        let row = sqlx::query_as::<_, StageRow>(
            "SELECT * FROM stages WHERE project_id = $1 AND position = $2",
        )
        .bind(project_id)
        .bind(position)
        .fetch_one(&self.pool)
        .await?;
        let base = Stage::new(row.project_id, row.position, row.title, row.deadline, row.completed);
        Ok(DetailedStage::new(base, row.description, row.cost))
    }

    pub async fn update_title(
        &self,
        project_id: Uuid,
        position: i32,
        title: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET title = $3 WHERE project_id = $1 AND position = $2")
            .bind(project_id)
            .bind(position)
            .bind(title)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_deadline(
        &self,
        project_id: Uuid,
        position: i32,
        deadline: Option<NaiveDateTime>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET deadline = $3 WHERE project_id = $1 AND position = $2")
            .bind(project_id)
            .bind(position)
            .bind(deadline)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_description(
        &self,
        project_id: Uuid,
        position: i32,
        description: Option<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE stages SET description = $3 WHERE project_id = $1 AND position = $2",
        )
        .bind(project_id)
        .bind(position)
        .bind(description)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_cost(
        &self,
        project_id: Uuid,
        position: i32,
        cost: Option<i32>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET cost = $3 WHERE project_id = $1 AND position = $2")
            .bind(project_id)
            .bind(position)
            .bind(cost)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_completed(
        &self,
        project_id: Uuid,
        position: i32,
        completed: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE stages SET completed = $3 WHERE project_id = $1 AND position = $2")
            .bind(project_id)
            .bind(position)
            .bind(completed)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn deadlines(&self) -> Result<Vec<DeadlineItem>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            project_id: Uuid,
            project_title: String,
            position: i32,
            stage_title: String,
            deadline: NaiveDateTime,
            completed: bool,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT s.project_id, p.title AS project_title, s.position,
                    s.title AS stage_title, s.deadline, s.completed
             FROM stages s
             JOIN projects p ON p.id = s.project_id
             WHERE s.deadline IS NOT NULL
             ORDER BY s.deadline",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| DeadlineItem {
                project_id: r.project_id,
                project_title: r.project_title,
                position: r.position,
                stage_title: r.stage_title,
                deadline: r.deadline,
                completed: r.completed,
            })
            .collect())
    }

    pub async fn remove(&self, project_id: Uuid, position: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM stages WHERE project_id = $1 AND position = $2")
            .bind(project_id)
            .bind(position)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
