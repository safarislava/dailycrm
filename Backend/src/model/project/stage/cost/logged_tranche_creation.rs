use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::comment::system_creation::SystemCommentCreation;
use crate::model::project::stage::cost::guarded_tranche_creation::GuardedTrancheCreation;
use crate::model::project::stage::cost::text::PaymentTrancheText;
use crate::model::project::stage::cost::valid_cost::ValidCost;
use crate::model::project::stage::id::StageId;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct LoggedTrancheCreation<T: PaymentTrancheText> {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    kind: &'static str,
    amount: ValidCost,
    text: T,
}

impl<T: PaymentTrancheText> LoggedTrancheCreation<T> {
    pub fn new(
        pool: Arc<PgPool>,
        stage_id: StageId,
        user_id: UserId,
        kind: &'static str,
        amount: ValidCost,
        text: T,
    ) -> Self {
        Self {
            pool,
            stage_id,
            user_id,
            kind,
            amount,
            text,
        }
    }
}

#[async_trait::async_trait]
impl<T: PaymentTrancheText> Task for LoggedTrancheCreation<T> {
    type Output = Uuid;

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let tranche_id = GuardedTrancheCreation::new(
            self.pool.clone(),
            self.stage_id,
            self.kind,
            self.amount,
            Some(self.user_id.id()),
        )
        .perform()
        .await?;

        let comment = self.text.creation_text(self.amount);
        let _ = SystemCommentCreation::new(self.pool.clone(), self.stage_id, self.user_id, comment)
            .perform()
            .await;

        Ok(tranche_id)
    }
}
