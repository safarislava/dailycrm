use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Stage {
    project_id: Uuid,
    position: i32,
    title: String,
    deadline: Option<DateTime<Utc>>,
    completed: bool,
}

impl Stage {
    pub fn new(
        project_id: Uuid,
        position: i32,
        title: String,
        deadline: Option<DateTime<Utc>>,
        completed: bool,
    ) -> Self {
        Self {
            project_id,
            position,
            title,
            deadline,
            completed,
        }
    }
}

#[derive(Serialize)]
pub struct StageWithProjectTitle {
    stage: Stage,
    project_title: String,
}

impl StageWithProjectTitle {
    pub fn new(stage: Stage, project_title: String) -> Self {
        Self {
            stage,
            project_title,
        }
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
        Self {
            stage,
            description,
            cost,
        }
    }
}
