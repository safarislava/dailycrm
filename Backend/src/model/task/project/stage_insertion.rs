use crate::common::BoxError;
use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageInsertion {
    pool: Arc<PgPool>,
    project: Project,
    position: i32,
    title: String,
}

impl StageInsertion {
    pub fn new(pool: Arc<PgPool>, project: Project, position: i32, title: String) -> Self {
        Self {
            pool,
            project,
            position,
            title,
        }
    }
}

#[async_trait::async_trait]
impl Task for StageInsertion {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            position: i32,
        }
        let mut transaction = self.pool.begin().await?;
        let rows = sqlx::query_as::<_, Row>(
            "SELECT position FROM stages \
             WHERE project_id = $1 AND position >= $2 ORDER BY position DESC",
        )
        .bind(self.project.id())
        .bind(self.position)
        .fetch_all(&mut *transaction)
        .await?;
        for row in rows {
            sqlx::query(
                "UPDATE stages SET position = position + 1 \
                 WHERE project_id = $1 AND position = $2",
            )
            .bind(self.project.id())
            .bind(row.position)
            .execute(&mut *transaction)
            .await?;
        }
        sqlx::query("INSERT INTO stages(project_id, position, title) VALUES ($1, $2, $3)")
            .bind(self.project.id())
            .bind(self.position)
            .bind(&self.title)
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        Ok(())
    }
}
