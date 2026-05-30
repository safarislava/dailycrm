use crate::auth::JwtToken;
use crate::model::session::refresh_token::RefreshToken;
use crate::model::task::session::refresh_token_revocation::RefreshTokenRevocation;
use crate::model::task::task::Task;
use crate::state::AppState;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn post(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    if let Some(cookie) = request.cookie("refresh_token") {
        if let Some(jti) = JwtToken::new(cookie.value()).jti() {
            let refresh_token = RefreshToken::new(jti, Box::new(cookie.value().to_string()));
            let _ = RefreshTokenRevocation::new(state.pool.clone(), refresh_token)
                .output()
                .await;
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
