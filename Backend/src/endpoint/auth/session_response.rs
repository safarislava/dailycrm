use crate::model::credential::contract::contentable::Contentable;
use crate::model::session::access_token::AccessToken;
use crate::model::session::cookie_token::CookieToken;
use crate::model::session::refresh_token::{REFRESH_LIFETIME, RefreshToken};
use actix_web::HttpResponse;
use actix_web::cookie::time::Duration;

pub struct SessionResponse {
    access: AccessToken,
    refresh: RefreshToken,
}

impl SessionResponse {
    pub fn new(access: AccessToken, refresh: RefreshToken) -> Self {
        Self { access, refresh }
    }

    pub async fn response(self) -> HttpResponse {
        let cookie = CookieToken::new(
            String::from("refresh_token"),
            Box::new(self.refresh),
            Duration::seconds(REFRESH_LIFETIME.num_seconds()),
        );
        match (self.access.content().await, cookie.content().await) {
            (Ok(access), Ok(cookie)) => HttpResponse::Ok()
                .cookie(cookie)
                .json(serde_json::json!({ "access_token": access })),
            _ => HttpResponse::Unauthorized().body("Invalid refresh token"),
        }
    }
}
