use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
pub struct User {
    id: Uuid,
    username: String,
    password_hash: String,
}
