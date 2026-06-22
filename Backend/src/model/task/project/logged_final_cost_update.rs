use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::comment_text::CommentText;
use crate::model::task::project::final_cost_change_text::FinalCostChangeText;
use crate::model::task::project::final_cost_update::FinalCostUpdate;
use crate::model::task::project::stage_final_cost_receipt::StageFinalCostReceipt;
use crate::model::task::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedFinalCostUpdate {
    pool: Arc<PgPool>,
    stage: Stage,
    user: User,
    cost: Option<i32>,
}

impl LoggedFinalCostUpdate {
    pub fn new(pool: Arc<PgPool>, stage: Stage, user: User, cost: Option<i32>) -> Self {
        Self {
            pool,
            stage,
            user,
            cost,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedFinalCostUpdate {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let old = StageFinalCostReceipt::new(self.pool.clone(), self.stage.clone())
            .done()
            .await?;
        FinalCostUpdate::new(self.pool.clone(), self.stage.clone(), self.cost)
            .done()
            .await?;
        if let Some(old_cost) = old {
            if self.cost != Some(old_cost) {
                let text = FinalCostChangeText::new(old_cost, self.cost).text();
                let _ = SystemCommentCreation::new(
                    self.pool.clone(),
                    self.stage.clone(),
                    self.user.clone(),
                    text,
                )
                .done()
                .await;
            }
        }
        Ok(())
    }
}