use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct PaymentConfirmation {
    pool: Arc<PgPool>,
    stage: Stage,
    confirmed: bool,
}

impl PaymentConfirmation {
    pub fn new(pool: Arc<PgPool>, stage: Stage, confirmed: bool) -> Self {
        Self {
            pool,
            stage,
            confirmed,
        }
    }
}

#[async_trait::async_trait]
impl Task for PaymentConfirmation {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query(
            "UPDATE stages SET payment_confirmed = $3 WHERE project_id = $1 AND position = $2",
        )
        .bind(self.stage.project().id())
        .bind(self.stage.position())
        .bind(self.confirmed)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
