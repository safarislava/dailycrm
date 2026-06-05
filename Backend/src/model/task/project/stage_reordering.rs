use crate::common::BoxError;
use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageReordering {
    pool: Arc<PgPool>,
    project: Project,
    parent_position: i32,
    from: i32,
    to: i32,
}

impl StageReordering {
    pub fn new(pool: Arc<PgPool>, project: Project, from: i32, to: i32) -> Self {
        Self { pool, project, parent_position: 0, from, to }
    }

    pub fn sub(pool: Arc<PgPool>, project: Project, parent_position: i32, from: i32, to: i32) -> Self {
        Self { pool, project, parent_position, from, to }
    }
}

#[async_trait::async_trait]
impl Task for StageReordering {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            max: Option<i32>,
        }
        let mut transaction = self.pool.begin().await?;
        let row: Row = sqlx::query_as(
            "SELECT MAX(position) AS max FROM stages WHERE project_id = $1 AND parent_position = $2",
        )
        .bind(self.project.id())
        .bind(self.parent_position)
        .fetch_one(&mut *transaction)
        .await?;
        let last = row.max.unwrap_or(0);
        if self.from < 1 || self.from > last {
            return Err(BoxError::from(sqlx::Error::RowNotFound));
        }
        let to = self.to.clamp(1, last);
        if self.from == to {
            transaction.commit().await?;
            return Ok(());
        }

        let lo = self.from.min(to);
        let hi = self.from.max(to);

        sqlx::query(
            "UPDATE stages SET position = -position \
             WHERE project_id = $1 AND parent_position = $2 AND position BETWEEN $3 AND $4",
        )
        .bind(self.project.id())
        .bind(self.parent_position)
        .bind(lo)
        .bind(hi)
        .execute(&mut *transaction)
        .await?;
        if self.parent_position == 0 {
            sqlx::query(
                "UPDATE stages SET parent_position = -parent_position \
                 WHERE project_id = $1 AND parent_position BETWEEN $2 AND $3",
            )
            .bind(self.project.id())
            .bind(lo)
            .bind(hi)
            .execute(&mut *transaction)
            .await?;
        }

        sqlx::query(
            "UPDATE stages SET position = $3 \
             WHERE project_id = $1 AND parent_position = $2 AND position = $4",
        )
        .bind(self.project.id())
        .bind(self.parent_position)
        .bind(to)
        .bind(-self.from)
        .execute(&mut *transaction)
        .await?;
        if self.parent_position == 0 {
            sqlx::query(
                "UPDATE stages SET parent_position = $2 WHERE project_id = $1 AND parent_position = $3",
            )
            .bind(self.project.id())
            .bind(to)
            .bind(-self.from)
            .execute(&mut *transaction)
            .await?;
        }

        let (lower, upper, delta) = if self.from < to {
            (-hi, -(lo + 1), -1)
        } else {
            (-(hi - 1), -lo, 1)
        };
        sqlx::query(
            "UPDATE stages SET position = -position + $5 \
             WHERE project_id = $1 AND parent_position = $2 AND position BETWEEN $3 AND $4",
        )
        .bind(self.project.id())
        .bind(self.parent_position)
        .bind(lower)
        .bind(upper)
        .bind(delta)
        .execute(&mut *transaction)
        .await?;
        if self.parent_position == 0 {
            sqlx::query(
                "UPDATE stages SET parent_position = -parent_position + $4 \
                 WHERE project_id = $1 AND parent_position BETWEEN $2 AND $3",
            )
            .bind(self.project.id())
            .bind(lower)
            .bind(upper)
            .bind(delta)
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }
}