use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::{Error, StatusCode};
use actix_web::HttpResponse;
use std::future::Future;
use std::pin::Pin;
use std::{
    future::{ready, Ready},
    task::Poll,
};

fn verify_token(token: &str) -> Result<bool, Error> {
    Ok(true)
}

type ServiceFuture = Pin<Box<dyn Future<Output = Result<ServiceResponse, ()>>>>;

pub struct AuthMW;

impl<S> Transform<S, ServiceRequest> for AuthMW
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = (), Future = ServiceFuture>,
{
    type Response = ServiceResponse;
    type Error = ();
    type InitError = ();
    type Transform = AuthMWService<S>;
    type Future = Ready<Result<Self::Transform, ()>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMWService { service }))
    }
}

pub struct AuthMWService<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMWService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = (), Future = ServiceFuture>,
{
    type Error = ();
    type Response = ServiceResponse;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, _: &mut core::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.path() == "/login" {
            return self.service.call(req);
        }
        if let Some(hv) = req.headers().get("X-Auth-Token") {
            if let Ok(token) = hv.to_str() {
                match verify_token(token) {
                    Ok(true) => {
                        return self.service.call(req);
                    }
                    _ => {
                        return Box::pin(ready(Ok(ServiceResponse::new(
                            req.request().clone(),
                            HttpResponse::new(StatusCode::UNAUTHORIZED),
                        ))))
                    }
                }
            }
            return Box::pin(ready(Ok(ServiceResponse::new(
                req.request().clone(),
                HttpResponse::new(StatusCode::UNAUTHORIZED),
            ))));
        }
        return Box::pin(ready(Ok(ServiceResponse::new(
            req.request().clone(),
            HttpResponse::new(StatusCode::UNAUTHORIZED),
        ))));
    }
}
