use crate::model::project::act::Act;
use crate::model::project::contract::list::List;
use crate::model::project::stage::Stage;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct Acts {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl Acts {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl List for Acts {
    type Output = Act;

    async fn items(&self) -> Result<Vec<Act>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: Uuid,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT id FROM attachments \
             WHERE project_id = $1 AND parent_position = $2 AND stage_position = $3 AND is_act = TRUE ORDER BY created_at",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows.into_iter().map(|row| Act::new(row.id)).collect())
    }
}