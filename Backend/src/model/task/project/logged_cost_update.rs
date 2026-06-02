use crate::common::BoxError;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
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
            if self.cost == Some(old_cost) {
                return Ok(());
            }
            let text = match self.cost {
                Some(new_cost) => format!(
                    "Стоимость изменена: {} ₽ → {} ₽",
                    format_cost(old_cost),
                    format_cost(new_cost),
                ),
                None => format!("Стоимость удалена: {} ₽", format_cost(old_cost)),
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

fn format_cost(cost: i32) -> String {
    let s = cost.to_string();
    let bytes = s.as_bytes();
    let mut result = String::new();
    let len = bytes.len();
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(' ');
        }
        result.push(b as char);
    }
    result
}
