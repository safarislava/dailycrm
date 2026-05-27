use crate::auth::JwtToken;
use crate::state::AppState;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn post(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    if let Some(cookie) = request.cookie("refresh_token") {
        if let Some(jti) = JwtToken::new(cookie.value()).refresh_jti() {
            let _ = state.refresh_tokens.revoke(jti).await;
        }
    }

    let expired = Cookie::build("refresh_token", "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/auth")
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .finish();

    HttpResponse::Ok().cookie(expired).finish()
}
