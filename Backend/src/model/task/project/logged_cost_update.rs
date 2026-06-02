use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::comment_text::CommentText;
use crate::model::task::project::cost_change_text::CostChangeText;
use crate::model::task::project::cost_update::CostUpdate;
use crate::model::task::project::stage_cost_receipt::StageCostReceipt;
use crate::model::task::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedCostUpdate {
    pool: Arc<PgPool>,
    stage: Stage,
    user: User,
    cost: Option<i32>,
}

impl LoggedCostUpdate {
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
impl Task for LoggedCostUpdate {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let old = StageCostReceipt::new(self.pool.clone(), self.stage.clone())
            .done()
            .await?;
        CostUpdate::new(self.pool.clone(), self.stage.clone(), self.cost)
            .done()
            .await?;
        if let Some(old_cost) = old {
            if self.cost != Some(old_cost) {
                let text = CostChangeText::new(old_cost, self.cost).text();
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