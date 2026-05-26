use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Project {
    id: Uuid,
    title: String,
    updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(id: Uuid, title: String, updated_at: DateTime<Utc>) -> Self {
        Project { id, title, updated_at }
    }
}