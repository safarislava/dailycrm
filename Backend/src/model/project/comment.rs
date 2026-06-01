use uuid::Uuid;

pub struct Comment {
    id: Uuid,
}

impl Comment {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}