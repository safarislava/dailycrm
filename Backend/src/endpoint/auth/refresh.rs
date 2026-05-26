use crate::auth::verify_token;
use crate::endpoint::auth::AuthResponse;
use crate::model::access_token::AccessToken;
use crate::model::refresh_token::RefreshToken;
use crate::state::AppState;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpRequest, HttpResponse, Responder, web};

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

    let user_id = match state.refresh_tokens.user_id_with_jti_revocation(claims.jti).await {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::Unauthorized().body("Token revoked or expired"),
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    let access_token = match AccessToken::new(user_id) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    let refresh_token = match RefreshToken::new(user_id) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    if state.refresh_tokens.store(&refresh_token).await.is_err() {
        return HttpResponse::InternalServerError().body("Something went wrong");
    }

    let new_cookie = Cookie::build("refresh_token", refresh_token.token_string)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/auth")
        .max_age(actix_web::cookie::time::Duration::days(7))
        .finish();

    HttpResponse::Ok()
        .cookie(new_cookie)
        .json(AuthResponse { access_token: access_token.token_string })
}