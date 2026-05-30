use crate::auth::JwtToken;
use actix_web::Error;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, forward_ready};
use std::pin::Pin;
use std::rc::Rc;

pub struct JwtMiddlewareService<S> {
    pub service: Rc<S>,
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
