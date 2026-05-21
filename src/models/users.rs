use crate::models::user::User;
use std::sync::Arc;

struct Users {
    users: Arc<[User]>,
}
