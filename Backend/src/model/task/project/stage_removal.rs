use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageRemoval {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl StageRemoval {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl Task for StageRemoval {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let mut transaction = self.pool.begin().await?;

        let result = sqlx::query(
            "DELETE FROM stages WHERE project_id = $1 AND parent_position = $2 AND position = $3",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .execute(&mut *transaction)
        .await?;

        if result.rows_affected() == 0 {
            return Err(BoxError::from(sqlx::Error::RowNotFound));
        }
        sqlx::query(
            "UPDATE stages SET position = -position \
             WHERE project_id = $1 AND parent_position = $2 AND position > $3",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .execute(&mut *transaction)
        .await?;

        if self.stage.parent_position() == 0 {
            sqlx::query(
                "UPDATE stages SET parent_position = -parent_position \
                 WHERE project_id = $1 AND parent_position > $2",
            )
            .bind(self.stage.project().id())
            .bind(self.stage.position())
            .execute(&mut *transaction)
            .await?;
        }

        sqlx::query(
            "UPDATE stages SET position = -position - 1 \
             WHERE project_id = $1 AND parent_position = $2 AND position < 0",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .execute(&mut *transaction)
        .await?;

        if self.stage.parent_position() == 0 {
            sqlx::query(
                "UPDATE stages SET parent_position = -parent_position - 1 \
                 WHERE project_id = $1 AND parent_position < 0",
            )
            .bind(self.stage.project().id())
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }
}
