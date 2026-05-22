use crate::repository::stage_repository::StageRow;
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
        Self { project_id, id, title }
    }
    
    pub fn new_from_row(row: StageRow) -> Self {
        Self::new(row.0, row.1, row.3)
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
    pub fn new(
        stage: Stage,
        description: String,
        deadline: DateTime<Local>,
        cost: i64,
    ) -> Self {
        Self {
            stage,
            description,
            deadline,
            cost,
        }
    }

    pub fn new_from_row(row: StageRow) -> Self {
        let stage = Stage::new_from_row(row.clone());
        Self::new(stage, row.4, row.5, row.6)
    }
}
