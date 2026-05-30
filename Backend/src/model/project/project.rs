use uuid::Uuid;

#[derive(Clone)]
pub struct Project {
    id: Uuid,
}

impl Project {
    pub fn new(id: Uuid) -> Self {
        Project { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
