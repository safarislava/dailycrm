use crate::auth::JwtToken;
use crate::model::access_token::AccessToken;
use crate::state::AppState;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn post(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    let cookie = match request.cookie("refresh_token") {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("No refresh token"),
    };

    let jti = match JwtToken::new(cookie.value()).refresh_jti() {
        Some(jti) => jti,
        None => return HttpResponse::Unauthorized().body("Invalid refresh token"),
    };

    let user_id = match state.refresh_tokens.token(jti).user_id_with_revocation().await {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::Unauthorized().body("Token revoked or expired"),
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    let access_token = AccessToken::new(user_id);

    let refresh_token = match state.refresh_tokens.new_token(user_id).await {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    let new_cookie = Cookie::build("refresh_token", refresh_token.encoded().to_owned())
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/auth")
        .max_age(actix_web::cookie::time::Duration::days(7))
        .finish();

    HttpResponse::Ok().cookie(new_cookie).json(access_token)
}