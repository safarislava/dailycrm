use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    id: Uuid,
}

impl User {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
