use crate::auth::{create_access_token, verify_token};
use crate::endpoint::auth::AuthResponse;
use crate::state::AppState;
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

    match state.refresh_tokens.is_valid(claims.jti).await {
        Ok(true) => {}
        Ok(false) => return HttpResponse::Unauthorized().body("Token revoked or expired"),
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    }

    match create_access_token(claims.sub) {
        Ok(access_token) => HttpResponse::Ok().json(AuthResponse { access_token }),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}