use crate::auth::JwtToken;
use actix_web::HttpRequest;
use uuid::Uuid;

pub trait AuthHeader {
    fn user_id(&self) -> Option<Uuid>;
}

impl AuthHeader for HttpRequest {
    fn user_id(&self) -> Option<Uuid> {
        self.headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .and_then(|token| JwtToken::new(token).access_user_id())
    }
}
