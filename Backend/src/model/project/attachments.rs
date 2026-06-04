use crate::model::project::attachment::Attachment;
use crate::model::project::contract::list::List;
use crate::model::project::stage::Stage;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct Attachments {
    pool: Arc<PgPool>,
    stage: Stage,
}

impl Attachments {
    pub fn new(pool: Arc<PgPool>, stage: Stage) -> Self {
        Self { pool, stage }
    }
}

#[async_trait::async_trait]
impl List for Attachments {
    type Output = Attachment;

    async fn items(&self) -> Result<Vec<Attachment>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct AttachmentRow {
            id: Uuid,
        }
        let rows = sqlx::query_as::<_, AttachmentRow>(
            "SELECT id FROM attachments \
            WHERE project_id = $1 AND parent_position = $2 AND stage_position = $3 AND is_act = FALSE ORDER BY created_at",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.parent_position())
        .bind(self.stage.position())
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Attachment::new(row.id))
            .collect())
    }
}
