use crate::model::contract::box_error::BoxError;
use crate::model::contract::comment_text::CommentText;
use crate::model::contract::task::Task;
use crate::model::contract::value::Value;
use crate::model::project::stage::comment::system_creation::SystemCommentCreation;
use crate::model::project::stage::cost::contract::cost::Cost;
use crate::model::project::stage::cost::r#final::change_text::FinalCostChangeText;
use crate::model::project::stage::cost::r#final::receipt::StageFinalCostReceipt;
use crate::model::project::stage::cost::r#final::update::FinalCostUpdate;
use crate::model::project::stage::id::StageId;
use crate::model::user::id::UserId;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedFinalCostUpdate {
    pool: Arc<PgPool>,
    stage_id: StageId,
    user_id: UserId,
    cost: Option<Box<dyn Cost>>,
}

impl LoggedFinalCostUpdate {
    pub fn new(
        pool: Arc<PgPool>,
        stage_id: StageId,
        user_id: UserId,
        cost: Option<Box<dyn Cost>>,
    ) -> Self {
        Self {
            pool,
            stage_id,
            user_id,
            cost,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedFinalCostUpdate {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let old = StageFinalCostReceipt::new(self.pool.clone(), self.stage_id)
            .value()
            .await?;
        FinalCostUpdate::new(self.pool.clone(), self.stage_id, self.cost.clone())
            .perform()
            .await?;
        if let Some(old_cost) = old
            && self.cost != Some(old_cost.clone())
        {
            let text = FinalCostChangeText::new(old_cost, self.cost.clone()).text();
            let _ =
                SystemCommentCreation::new(self.pool.clone(), self.stage_id, self.user_id, text)
                    .perform()
                    .await;
        }
        Ok(())
    }
}
