use uuid::Uuid;

pub trait Invite {
    fn token(&self) -> Uuid;
}