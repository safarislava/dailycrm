use crate::model::user::User;
use std::sync::Arc;

struct Users {
    users: Arc<[User]>,
}
