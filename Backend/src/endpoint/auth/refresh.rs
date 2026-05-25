use crate::auth::{create_access_token, create_refresh_token, verify_token};
use crate::endpoint::auth::AuthResponse;
use crate::state::AppState;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use chrono::{Duration, Utc};
use uuid::Uuid;

pub async fn post(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    let cookie = match request.cookie("refresh_token") {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("No refresh token"),
    };

    let claims = match verify_token(cookie.value()) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid refresh token"),
    };

    if claims.typ != "refresh" {
        return HttpResponse::Unauthorized().body("Invalid refresh token");
    }

    let user_id = match state.refresh_tokens.validate_and_revoke(claims.jti).await {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::Unauthorized().body("Token revoked or expired"),
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    let new_jti = Uuid::new_v4();
    let expires_at = Utc::now() + Duration::days(7);

    if state.refresh_tokens.store(new_jti, user_id, expires_at).await.is_err() {
        return HttpResponse::InternalServerError().body("Something went wrong");
    }

    let (access_token, refresh_token) =
        match (create_access_token(user_id), create_refresh_token(user_id, new_jti)) {
            (Ok(at), Ok(rt)) => (at, rt),
            _ => return HttpResponse::InternalServerError().body("Something went wrong"),
        };

    let new_cookie = Cookie::build("refresh_token", refresh_token)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/auth")
        .max_age(actix_web::cookie::time::Duration::days(7))
        .finish();

    HttpResponse::Ok()
        .cookie(new_cookie)
        .json(AuthResponse { access_token })
}