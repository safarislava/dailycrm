use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::session::contract::token::Token;
use actix_web::cookie::time::Duration;
use actix_web::cookie::{Cookie, SameSite};

pub struct CookieToken {
    name: String,
    token: Box<dyn Token>,
    max_age: Duration,
}

impl CookieToken {
    pub fn new(name: String, token: Box<dyn Token>, max_age: Duration) -> Self {
        Self {
            name,
            token,
            max_age,
        }
    }
}

#[async_trait::async_trait]
impl Contentable for CookieToken {
    type Output = Cookie<'static>;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        Ok(
            Cookie::build(self.name.clone(), self.token.content().await?)
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Strict)
                .path("/api/auth")
                .max_age(self.max_age)
                .finish(),
        )
    }
}
