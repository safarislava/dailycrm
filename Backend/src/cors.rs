use actix_cors::Cors;
use actix_web::http;
use actix_web::middleware::DefaultHeaders;
use std::env;

pub fn rules() -> Cors {
    let allowed_origin =
        &env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());
    Cors::default()
        .allowed_origin(allowed_origin)
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::CONTENT_TYPE,
        ])
        .supports_credentials()
        .max_age(3600)
}

pub fn security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .add(("X-Content-Type-Options", "nosniff"))
        .add(("X-Frame-Options", "DENY"))
        .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
}
