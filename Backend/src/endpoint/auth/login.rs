use crate::model::user::LoginError;
use crate::state::AppState;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginDto {
    username: String,
    password: String,
}

pub async fn post(state: web::Data<AppState>, body: web::Json<LoginDto>) -> impl Responder {
    let user = match state.users.user_by_username(&body.username).await {
        Ok(Some(u)) => u,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    let (access_token, refresh_token) = match user.tokens(&body.password).await {
        Ok(tokens) => tokens,
        Err(LoginError::WrongPassword) => {
            return HttpResponse::Unauthorized().body("Invalid credentials");
        }
        Err(LoginError::Internal) => {
            return HttpResponse::InternalServerError().body("Something went wrong");
        }
    };

    let refresh_token_string = match refresh_token.store(&state.pool).await {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    let cookie = Cookie::build("refresh_token", refresh_token_string)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/auth")
        .max_age(actix_web::cookie::time::Duration::days(7))
        .finish();

    HttpResponse::Ok().cookie(cookie).json(access_token)
}
