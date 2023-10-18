use crate::error::Error;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
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

pub struct AuthMW {
    url: String,
}

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
        ready(Ok(AuthMWService {
            url: self.url.clone(),
            service,
        }))
    }
}

pub struct AuthMWService<S> {
    url: String,
    service: S,
}

impl<S> AuthMWService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = (), Future = ServiceFuture>,
{
    async fn verify_token(&self, token: &str) -> Result<(), Error> {
        let resp = reqwest::Client::new()
            .get(format!("{}/{}", &self.url, token))
            .send()
            .await
            .map_err(|e| {
                Error::wrap(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to communicate with auth service",
                    e,
                )
            })?;
        let status = resp.status();
        if status != StatusCode::OK {
            match resp.text().await {
                Ok(text) => return Err(Error::new(status, text)),
                Err(e) => {
                    return Err(Error::wrap(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to parse response",
                        e,
                    ))
                }
            }
        }
        Ok(())
    }
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
                let next = self.service.call(req);
                return Box::pin(async move {
                    match self.verify_token(token).await {
                        Ok(()) => {
                            return self.service.call(req);
                        }
                        _ => {
                            return Box::pin(ready(Ok(ServiceResponse::new(
                                req.request().clone(),
                                HttpResponse::new(StatusCode::UNAUTHORIZED),
                            ))))
                        }
                    }
                });
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
