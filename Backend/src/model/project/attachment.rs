use uuid::Uuid;

#[derive(Clone)]
pub struct Attachment {
    id: Uuid,
}

impl Attachment {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
