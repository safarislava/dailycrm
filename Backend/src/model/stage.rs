use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Stage {
    project_id: Uuid,
    position: i32,
    title: String,
    deadline: Option<NaiveDateTime>,
    completed: bool,
}

impl Stage {
    pub fn new(project_id: Uuid, position: i32, title: String, deadline: Option<NaiveDateTime>, completed: bool) -> Self {
        Self { project_id, position, title, deadline, completed }
    }
}

#[derive(Serialize)]
pub struct DeadlineItem {
    pub project_id: Uuid,
    pub project_title: String,
    pub position: i32,
    pub stage_title: String,
    pub deadline: NaiveDateTime,
    pub completed: bool,
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
