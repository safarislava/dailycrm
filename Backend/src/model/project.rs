use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Project {
    id: Uuid,
    title: String,
    updated_at: NaiveDateTime,
}

impl Project {
    pub fn new(id: Uuid, title: String, updated_at: NaiveDateTime) -> Self {
        Project { id, title, updated_at }
    }
}
