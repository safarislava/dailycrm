use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::notification::notification_enqueue::NotificationEnqueue;
use crate::model::task::project::gip_confirmation::GipConfirmation;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotifiedGipConfirmation {
    pool: Arc<PgPool>,
    stage: Stage,
    confirmed: bool,
}

impl NotifiedGipConfirmation {
    pub fn new(pool: Arc<PgPool>, stage: Stage, confirmed: bool) -> Self {
        Self {
            pool,
            stage,
            confirmed,
        }
    }
}

#[async_trait::async_trait]
impl Task for NotifiedGipConfirmation {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        GipConfirmation::new(self.pool.clone(), self.stage.clone(), self.confirmed)
            .done()
            .await?;
        if self.confirmed {
            NotificationEnqueue::new(self.pool.clone(), self.stage.clone(), "work_complete")
                .done()
                .await?;
        }
        Ok(())
    }
}