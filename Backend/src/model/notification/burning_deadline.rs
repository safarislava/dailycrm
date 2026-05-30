use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use chrono::{DateTime, Utc};

pub struct BurningDeadline {
    project_title: String,
    stage_title: String,
    deadline: DateTime<Utc>,
}

impl BurningDeadline {
    pub fn new(project_title: String, stage_title: String, deadline: DateTime<Utc>) -> Self {
        Self {
            project_title,
            stage_title,
            deadline,
        }
    }
}

#[async_trait::async_trait]
impl Contentable for BurningDeadline {
    type Output = String;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        Ok(format!(
            "• {} / {} — {}",
            self.project_title,
            self.stage_title,
            self.deadline.format("%d-%m-%Y")
        ))
    }
}
