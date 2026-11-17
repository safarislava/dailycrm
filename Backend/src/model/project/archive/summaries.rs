use crate::model::contract::box_error::BoxError;
use crate::model::contract::printer::Printer;
use crate::model::contract::project_media::ProjectMedia;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct ArchivedProjectSummaries {
    pool: Arc<PgPool>,
}

impl ArchivedProjectSummaries {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<M: ProjectMedia> Printer<M> for ArchivedProjectSummaries {
    async fn print(&self, media: &mut M) -> Result<(), BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: Uuid,
            title: String,
            archived_at: DateTime<Utc>,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT id, title, archived_at FROM archived_projects ORDER BY archived_at DESC",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        for r in rows {
            media.add_project(r.id, &r.title, r.archived_at);
        }
        Ok(())
    }
}
