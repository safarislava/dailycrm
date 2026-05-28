use crate::model::authorized_user::LoginError;
use crate::model::password::Password;
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

    let password = match Password(body.password.clone()).validated() {
        Ok(p) => p,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid credentials"),
    };
    let (access_token, refresh_token) = match user
        .confirming(password)
        .tokens(state.refresh_tokens.as_ref())
        .await
    {
        Ok(tokens) => tokens,
        Err(LoginError::WrongPassword) => {
            return HttpResponse::Unauthorized().body("Invalid credentials");
        }
        Err(LoginError::Internal) => {
            return HttpResponse::InternalServerError().body("Something went wrong");
        }
    };

    let cookie = Cookie::build("refresh_token", refresh_token.encoded().to_owned())
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/auth")
        .max_age(actix_web::cookie::time::Duration::days(7))
        .finish();

    HttpResponse::Ok().cookie(cookie).json(access_token)
}
