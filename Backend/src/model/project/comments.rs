use crate::model::project::comment::Comment;
use crate::model::project::contract::list::List;
use crate::model::project::stage::Stage;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct Comments {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl Comments {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl List for Comments {
    type Output = Comment;

    async fn items(&self) -> Result<Vec<Comment>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: Uuid,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT id FROM stage_comments \
             WHERE project_id = $1 AND stage_position = $2 ORDER BY created_at",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.position())
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows.into_iter().map(|r| Comment::new(r.id)).collect())
    }
}
