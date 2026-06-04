use crate::common::BoxError;
use crate::model::project::attachment::Attachment;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AttachmentReceipt {
    pool: Arc<PgPool>,
    attachment: Attachment,
}

impl AttachmentReceipt {
    pub fn new(pool: Arc<PgPool>, attachment: Attachment) -> Self {
        Self { pool, attachment }
    }
}

#[async_trait::async_trait]
impl Task for AttachmentReceipt {
    type Output = Option<(String, Stage, bool)>;

    async fn done(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            project_id: Uuid,
            parent_position: i32,
            stage_position: i32,
            filename: String,
            is_act: bool,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT project_id, parent_position, stage_position, filename, is_act FROM attachments WHERE id = $1",
        )
        .bind(self.attachment.id())
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(row.map(|r| {
            let stage = Stage::new_substage(Project::new(r.project_id), r.parent_position, r.stage_position);
            (r.filename, stage, r.is_act)
        }))
    }
}
