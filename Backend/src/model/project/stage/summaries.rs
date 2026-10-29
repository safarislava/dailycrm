use crate::model::contract::box_error::BoxError;
use crate::model::contract::printer::Printer;
use crate::model::contract::stage_media::StageMedia;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::collecting_media::StageSummaryItem;
use sqlx::PgPool;
use std::sync::Arc;

pub struct StageSummaries {
    pool: Arc<PgPool>,
    project_id: ProjectId,
}

impl StageSummaries {
    pub fn new(pool: Arc<PgPool>, project_id: ProjectId) -> Self {
        Self { pool, project_id }
    }
}

#[async_trait::async_trait]
impl<M: StageMedia> Printer<M> for StageSummaries {
    async fn print(&self, media: &mut M) -> Result<(), BoxError> {
        let items = sqlx::query_as::<_, StageSummaryItem>(
            "SELECT s.project_id, s.parent_position, s.position, s.title, s.deadline, s.completed,
                    s.gip_confirmed, s.advance_cost, s.advance_confirmed,
                    s.second_advance_cost, s.second_advance_confirmed,
                    s.final_cost, s.final_confirmed,
                    EXISTS(
                       SELECT 1 FROM attachments a
                       WHERE a.project_id = s.project_id
                         AND a.parent_position = s.parent_position
                         AND a.stage_position = s.position
                         AND a.is_act = TRUE
                    ) AS has_act
             FROM detailed_stages s
             WHERE s.project_id = $1 ORDER BY s.parent_position, s.position",
        )
        .bind(self.project_id.id())
        .fetch_all(self.pool.as_ref())
        .await?;
        for item in &items {
            item.print(media);
        }
        Ok(())
    }
}
