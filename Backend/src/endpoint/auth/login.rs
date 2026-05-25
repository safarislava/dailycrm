use crate::auth::{create_access_token, create_refresh_token};
use crate::endpoint::auth::AuthResponse;
use crate::model::user::{ValidPasswordHash, VerifyError};
use crate::state::AppState;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpResponse, Responder, web};
use chrono::{Duration, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginDto {
    username: String,
    password: String,
}

pub async fn post(state: web::Data<AppState>, body: web::Json<LoginDto>) -> impl Responder {
    let (user_id, stored_hash) = match state.users.find_by_username(&body.username).await {
        Ok(Some(u)) => u,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    match ValidPasswordHash::try_new(stored_hash, &body.password).await {
        Ok(_) => {}
        Err(VerifyError::WrongPassword) => return HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(VerifyError::Internal) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    let jti = Uuid::new_v4();
    let expires_at = Utc::now() + Duration::days(7);

    let (access_token, refresh_token) =
        match (create_access_token(user_id), create_refresh_token(user_id, jti)) {
            (Ok(at), Ok(rt)) => (at, rt),
            _ => return HttpResponse::InternalServerError().body("Something went wrong"),
        };

    if state.refresh_tokens.store(jti, user_id, expires_at).await.is_err() {
        return HttpResponse::InternalServerError().body("Something went wrong");
    }

    let cookie = Cookie::build("refresh_token", refresh_token)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/auth")
        .max_age(actix_web::cookie::time::Duration::days(7))
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(AuthResponse { access_token })
}