use crate::common::BoxError;
use crate::model::session::contract::cookie::Cookie;
use crate::model::session::contract::token::Token;
use actix_web::cookie::time::Duration;

pub struct CookieToken {
    name: String,
    token: Box<dyn Token>,
    max_age: Duration,
}

impl CookieToken {
    pub fn new(name: String, token: Box<dyn Token>, max_age: Duration) -> Self {
        Self { name, token, max_age }
    }
}

#[async_trait::async_trait]
impl Cookie for CookieToken {
    async fn value(&self) -> Result<actix_web::cookie::Cookie<'static>, BoxError> {
        Ok(
            actix_web::cookie::Cookie::build(self.name.clone(), self.token.value().await?)
                .http_only(true)
                .secure(true)
                .same_site(actix_web::cookie::SameSite::Strict)
                .path("/api/auth")
                .max_age(self.max_age)
                .finish(),
        )
    }
}