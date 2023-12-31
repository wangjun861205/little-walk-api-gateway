use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use reqwest::header::{HeaderName, HeaderValue};
use std::future::Future;
use std::pin::Pin;
use std::{
    future::{ready, Ready},
    task::Poll,
};

use crate::core::clients::auth::AuthClient;
use std::str::FromStr;
use std::sync::Arc;

type ServiceFuture =
    Pin<Box<dyn Future<Output = Result<ServiceResponse, Error>>>>;

#[derive(Clone)]
pub struct AuthMiddlewareFactory<C>
where
    C: AuthClient + Clone,
{
    auth_client: C,
}

impl<C> AuthMiddlewareFactory<C>
where
    C: AuthClient + Clone,
{
    pub fn new(auth_client: C) -> Self {
        Self { auth_client }
    }
}

impl<C, S> Transform<S, ServiceRequest> for AuthMiddlewareFactory<C>
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse,
            Error = Error,
            Future = ServiceFuture,
        > + 'static,
    C: AuthClient + Clone + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<C, S>;
    type Future = Ready<Result<Self::Transform, ()>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            auth_client: self.auth_client.clone(),
            service: Arc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<C, S>
where
    C: AuthClient + Clone,
{
    auth_client: C,
    service: Arc<S>,
}

impl<C, S> Service<ServiceRequest> for AuthMiddlewareService<C, S>
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse,
            Error = Error,
            Future = ServiceFuture,
        > + 'static,
    C: AuthClient + Clone + 'static,
{
    type Error = Error;
    type Response = ServiceResponse;
    type Future = ServiceFuture;

    fn poll_ready(
        &self,
        _: &mut core::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        if let Some(hv) = req.headers().get("X-Auth-Token") {
            if let Ok(token) = hv.to_str() {
                let token = token.to_owned();
                let next_service = self.service.clone();
                let auth_client = self.auth_client.clone();
                return Box::pin(async move {
                    match auth_client.verify_token(&token).await {
                        Ok(id) => {
                            let uid_header_key =
                                HeaderName::from_str("X-User-ID")
                                    .map_err(ErrorInternalServerError)?;
                            let uid_header_value =
                                HeaderValue::from_str(&id)
                                    .map_err(ErrorInternalServerError)?;
                            req.headers_mut()
                                .insert(uid_header_key, uid_header_value);
                            next_service.call(req).await
                        }
                        Err(e) => Err(ErrorUnauthorized(e)),
                    }
                });
            }
        }
        Box::pin(ready(Err(ErrorUnauthorized("auth token not exists"))))
    }
}
