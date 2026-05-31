use crate::model::session::access_token_decoder::AccessTokenDecoder;
use crate::model::session::contract::access_token_decodable::AccessTokenDecodable;
use crate::model::user::user::User;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, forward_ready};
use actix_web::{Error, HttpMessage};
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

        let user_id = request
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .and_then(|token| AccessTokenDecoder::new(token.to_string()).user_id());

        match user_id {
            Some(id) => {
                request.extensions_mut().insert(User::new(id));
                Box::pin(svc.call(request))
            }
            None => Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) }),
        }
    }
}
