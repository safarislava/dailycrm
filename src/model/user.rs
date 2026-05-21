use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    id: Uuid,
    username: String,
    password_hash: String,
}
