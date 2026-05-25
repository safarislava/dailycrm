use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::{
    env,
    future::{Future, Ready, ready},
    pin::Pin,
    rc::Rc,
    sync::OnceLock,
};
use uuid::Uuid;

static JWT_SECRET: OnceLock<String> = OnceLock::new();

fn jwt_secret() -> &'static str {
    JWT_SECRET.get_or_init(|| env::var("JWT_SECRET").expect("JWT_SECRET must be set"))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub jti: Uuid,
    pub typ: String,
    pub exp: usize,
}

pub fn create_access_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (Utc::now().timestamp() + 15 * 60) as usize;
    encode(
        &Header::default(),
        &Claims { sub: user_id, jti: Uuid::new_v4(), typ: "access".into(), exp },
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

pub fn create_refresh_token(
    user_id: Uuid,
    jti: Uuid,
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (Utc::now().timestamp() + 7 * 24 * 3600) as usize;
    encode(
        &Header::default(),
        &Claims { sub: user_id, jti, typ: "refresh".into(), exp },
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

pub fn user_id_from_request(request: &actix_web::HttpRequest) -> Option<Uuid> {
    request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .and_then(|token| verify_token(token).ok())
        .filter(|claims| claims.typ == "access")
        .map(|claims| claims.sub)
}

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        let authorized = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .and_then(|token| verify_token(token).ok())
            .map(|claims| claims.typ == "access")
            .unwrap_or(false);

        Box::pin(async move {
            if authorized {
                svc.call(req).await
            } else {
                Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
            }
        })
    }
}