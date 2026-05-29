use actix_cors::Cors;
use actix_web::http;
use actix_web::middleware::DefaultHeaders;

pub fn cors(allowed_origin: &str) -> Cors {
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
