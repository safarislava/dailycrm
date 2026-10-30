use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::stage::comment::system_creation::SystemCommentCreation;
use crate::model::project::stage::cost::text::PaymentTrancheText;
use crate::model::project::stage::cost::tranche_deletion::TrancheDeletion;
use crate::model::project::stage::id::StageId;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct LoggedTrancheDeletion<T: PaymentTrancheText> {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    kind: &'static str,
    tranche_id: Uuid,
    text: T,
}

impl<T: PaymentTrancheText> LoggedTrancheDeletion<T> {
    pub fn new(
        pool: Arc<PgPool>,
        stage_id: StageId,
        user_id: UserId,
        kind: &'static str,
        tranche_id: Uuid,
        text: T,
    ) -> Self {
        Self {
            pool,
            stage_id,
            user_id,
            kind,
            tranche_id,
            text,
        }
    }
}

#[async_trait::async_trait]
impl<T: PaymentTrancheText> Task for LoggedTrancheDeletion<T> {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let deleted_amount =
            TrancheDeletion::new(self.pool.clone(), self.stage_id, self.kind, self.tranche_id)
                .perform()
                .await?;

        let comment = self.text.deletion_text(deleted_amount);
        let _ = SystemCommentCreation::new(self.pool.clone(), self.stage_id, self.user_id, comment)
            .perform()
            .await;

        Ok(())
    }
}
