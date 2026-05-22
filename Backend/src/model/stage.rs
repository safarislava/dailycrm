use chrono::{DateTime, Local};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Stage {
    project_id: Uuid,
    position: i32,
    title: String,
    deadline: Option<DateTime<Local>>,
}

impl Stage {
    pub fn new(project_id: Uuid, position: i32, title: String, deadline: Option<DateTime<Local>>) -> Self {
        Self { project_id, position, title, deadline }
    }
}

#[derive(Serialize)]
pub struct DetailedStage {
    stage: Stage,
    description: Option<String>,
    cost: Option<i32>,
}

impl DetailedStage {
    pub fn new(stage: Stage, description: Option<String>, cost: Option<i32>) -> Self {
        Self { stage, description, cost }
    }
}
