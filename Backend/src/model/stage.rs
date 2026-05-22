use chrono::{DateTime, Local};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Stage {
    project_id: Uuid,
    id: Uuid,
    title: String,
}

impl Stage {
    pub fn new(project_id: Uuid, id: Uuid, title: String) -> Self {
        Self {
            project_id,
            id,
            title,
        }
    }
}

#[derive(Serialize)]
pub struct DetailedStage {
    stage: Stage,
    description: String,
    deadline: DateTime<Local>,
    cost: i64,
}

impl DetailedStage {
    pub fn new(stage: Stage, description: String, deadline: DateTime<Local>, cost: i64) -> Self {
        Self {
            stage,
            description,
            deadline,
            cost,
        }
    }
}
