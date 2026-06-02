use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::deadline_update::DeadlineUpdate;
use crate::model::task::project::stage_deadline_receipt::StageDeadlineReceipt;
use crate::model::task::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedDeadlineUpdate {
    pool: Arc<PgPool>,
    stage: Stage,
    user: User,
    deadline: Option<DateTime<Utc>>,
}

impl LoggedDeadlineUpdate {
    pub fn new(
        pool: Arc<PgPool>,
        stage: Stage,
        user: User,
        deadline: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            pool,
            stage,
            user,
            deadline,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedDeadlineUpdate {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let old = StageDeadlineReceipt::new(self.pool.clone(), self.stage.clone())
            .done()
            .await?;
        DeadlineUpdate::new(self.pool.clone(), self.stage.clone(), self.deadline)
            .done()
            .await?;
        if let Some(old_date) = old {
            if self.deadline == Some(old_date) {
                return Ok(());
            }
            let text = match self.deadline {
                Some(new_date) => format!(
                    "Дедлайн изменён: {} → {}",
                    old_date.format("%d.%m.%Y"),
                    new_date.format("%d.%m.%Y"),
                ),
                None => format!("Дедлайн удалён: {}", old_date.format("%d.%m.%Y")),
            };
            let _ = SystemCommentCreation::new(
                self.pool.clone(),
                self.stage.clone(),
                self.user.clone(),
                text,
            )
            .done()
            .await;
        }
        Ok(())
    }
}
