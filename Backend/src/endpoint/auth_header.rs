use crate::model::user::user::User;
use actix_web::{HttpMessage, HttpRequest};

pub trait UserHeader {
    fn user(&self) -> Option<User>;
}

impl UserHeader for HttpRequest {
    fn user(&self) -> Option<User> {
        self.extensions().get::<User>().map(|user| user.clone())
    }
}
