use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpResponse, Responder};

pub async fn post() -> impl Responder {
    let cookie = Cookie::build("refresh_token", "")
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/api/auth")
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .finish();

    HttpResponse::Ok().cookie(cookie).finish()
}