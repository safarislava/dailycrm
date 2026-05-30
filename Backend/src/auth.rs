use actix_web::HttpRequest;
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use crate::model::session::token_kind::TokenKind;
use jsonwebtoken::{DecodingKey, Validation, decode};
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

pub fn jwt_secret() -> &'static str {
    JWT_SECRET.get_or_init(|| env::var("JWT_SECRET").expect("JWT_SECRET must be set"))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub jti: Uuid,
    pub typ: String,
    pub exp: usize,
}

pub struct JwtToken {
    raw: String,
}

impl JwtToken {
    pub fn new(raw: &str) -> Self {
        Self {
            raw: raw.to_owned(),
        }
    }

    pub fn access_user_id(&self) -> Option<Uuid> {
        self.decode()
            .ok()
            .filter(|c| c.typ == TokenKind::Access.as_str())
            .map(|c| c.sub)
    }

    pub fn jti(&self) -> Option<Uuid> {
        self.decode()
            .ok()
            .filter(|c| c.typ == TokenKind::Refresh.as_str())
            .map(|c| c.jti)
    }

    fn decode(&self) -> Result<Claims, jsonwebtoken::errors::Error> {
        let data = decode::<Claims>(
            &self.raw,
            &DecodingKey::from_secret(jwt_secret().as_bytes()),
            &Validation::default(),
        )?;
        Ok(data.claims)
    }
}

pub trait UserIdGettable {
    fn user_id(&self) -> Option<Uuid>;
}

impl UserIdGettable for HttpRequest {
    fn user_id(&self) -> Option<Uuid> {
        self.headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .and_then(|token| JwtToken::new(token).access_user_id())
    }
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

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        let authorized = request
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|token| JwtToken::new(token).access_user_id().is_some())
            .unwrap_or(false);

        Box::pin(async move {
            if authorized {
                svc.call(request).await
            } else {
                Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
            }
        })
    }
}
