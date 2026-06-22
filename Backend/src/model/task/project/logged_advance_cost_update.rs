use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::advance_cost_change_text::AdvanceCostChangeText;
use crate::model::task::project::advance_cost_update::AdvanceCostUpdate;
use crate::model::task::project::comment_text::CommentText;
use crate::model::task::project::stage_advance_cost_receipt::StageAdvanceCostReceipt;
use crate::model::task::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedAdvanceCostUpdate {
    pool: Arc<PgPool>,
    stage: Stage,
    user: User,
    cost: Option<i32>,
}

impl LoggedAdvanceCostUpdate {
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
impl Task for LoggedAdvanceCostUpdate {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let old = StageAdvanceCostReceipt::new(self.pool.clone(), self.stage.clone())
            .done()
            .await?;
        AdvanceCostUpdate::new(self.pool.clone(), self.stage.clone(), self.cost)
            .done()
            .await?;
        if let Some(old_cost) = old {
            if self.cost != Some(old_cost) {
                let text = AdvanceCostChangeText::new(old_cost, self.cost).text();
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