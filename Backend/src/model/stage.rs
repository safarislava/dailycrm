use chrono::{DateTime, Local};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Stage {
    project_id: Uuid,
    position: i32,
    title: String,
}

impl Stage {
    pub fn new(project_id: Uuid, position: i32, title: String) -> Self {
        Self {
            project_id,
            position,
            title,
        }
    }
}

#[derive(Serialize)]
pub struct DetailedStage {
    stage: Stage,
    description: Option<String>,
    deadline: Option<DateTime<Local>>,
    cost: Option<i32>,
}

impl DetailedStage {
    pub fn new(
        stage: Stage,
        description: Option<String>,
        deadline: Option<DateTime<Local>>,
        cost: Option<i32>,
    ) -> Self {
        Self { stage, description, deadline, cost }
    }
}
