use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Project {
    id: Uuid,
    title: String,
}

impl Project {
    pub fn new(id: Uuid, title: String) -> Self {
        Project { id, title }
    }
}
