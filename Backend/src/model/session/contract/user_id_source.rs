use uuid::Uuid;

pub trait UserIdSource {
    fn user_id(&self) -> Option<Uuid>;
}
