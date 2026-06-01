use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::notification::notification_enqueue::NotificationEnqueue;
use sqlx::PgPool;
use std::sync::Arc;

pub struct GipConfirmation {
    pool: Arc<PgPool>,
    stage: Stage,
    confirmed: bool,
}

impl GipConfirmation {
    pub fn new(pool: Arc<PgPool>, stage: Stage, confirmed: bool) -> Self {
        Self { pool, stage, confirmed }
    }
}

#[async_trait::async_trait]
impl Task for GipConfirmation {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("UPDATE stages SET gip_confirmed = $3 WHERE project_id = $1 AND position = $2")
            .bind(self.stage.project().id())
            .bind(self.stage.position())
            .bind(self.confirmed)
            .execute(self.pool.as_ref())
            .await?;
        if self.confirmed {
            NotificationEnqueue::new(self.pool.clone(), self.stage.clone(), "work_complete")
                .done()
                .await?;
        }
        Ok(())
    }
}
