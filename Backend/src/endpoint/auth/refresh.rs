use crate::endpoint::auth::session_response::SessionResponse;
use crate::model::session::contract::refresh_token_decodable::RefreshTokenDecodable;
use crate::model::session::refresh_token::RefreshToken;
use crate::model::session::refresh_token_decoder::RefreshTokenDecoder;
use crate::model::task::contract::task::Task;
use crate::model::task::user::tokens_issuance::TokenIssuance;
use crate::model::user::jwt_protected_user::JwtProtectedUser;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};

pub async fn post(state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    let cookie = match request.cookie("refresh_token") {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("No refresh token"),
    };
    let jti = match RefreshTokenDecoder::new(cookie.value().to_string()).jti() {
        Some(jti) => jti,
        None => return HttpResponse::Unauthorized().body("Invalid refresh token"),
    };
    let refresh_token = RefreshToken::new(jti, Box::new(cookie.value().to_string()));
    let user = JwtProtectedUser::new(state.pool.clone(), refresh_token);
    let task = TokenIssuance::new(state.pool.clone(), Box::new(user));
    match task.done().await {
        Ok(Some((access, refresh))) => SessionResponse::new(access, refresh).response().await,
        Ok(None) => HttpResponse::Unauthorized().body("Token revoked or expired"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
