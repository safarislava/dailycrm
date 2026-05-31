use uuid::Uuid;

pub struct Act {
    id: Uuid,
}

impl Act {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}