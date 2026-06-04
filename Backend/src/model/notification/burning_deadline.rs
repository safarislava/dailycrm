use crate::common::BoxError;
use crate::model::notification::contract::message::Message;
use chrono::{DateTime, Utc};

pub struct BurningDeadline {
    project_title: String,
    stage_title: String,
    deadline: DateTime<Utc>,
}

impl BurningDeadline {
    pub fn new(project_title: String, stage_title: String, deadline: DateTime<Utc>) -> Self {
        Self { project_title, stage_title, deadline }
    }
}

#[async_trait::async_trait]
impl Message for BurningDeadline {
    async fn text(&self) -> Result<String, BoxError> {
        Ok(format!(
            "• {} / {} — {}",
            self.project_title,
            self.stage_title,
            self.deadline.format("%d-%m-%Y")
        ))
    }
}