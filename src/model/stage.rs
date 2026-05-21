use crate::repository::stage_repository::StageRow;
use chrono::{DateTime, Local};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Stage {
    project_id: Uuid,
    id: Uuid,
    title: String,
    description: String,
    deadline: DateTime<Local>,
    cost: i64,
}

impl Stage {
    pub fn new(
        project_id: Uuid,
        id: Uuid,
        title: String,
        description: String,
        deadline: DateTime<Local>,
        cost: i64,
    ) -> Self {
        Self {
            project_id,
            id,
            title,
            description,
            deadline,
            cost,
        }
    }

    pub fn new_from_row(row: StageRow) -> Self {
        Self::new(row.0, row.1, row.3, row.4, row.5, row.6)
    }
}
