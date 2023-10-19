use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use actix_web::http::StatusCode;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use std::future::Future;
use std::pin::Pin;
use std::{
    future::{ready, Ready},
    task::Poll,
};

type ServiceFuture = Pin<Box<dyn Future<Output = Result<ServiceResponse, Error>>>>;

pub struct AuthMW {
    auth_service_address: String,
}

impl AuthMW {
    pub fn new(auth_service_address: String) -> Self {
        Self {
            auth_service_address,
        }
    }
}

impl<S> Transform<S, ServiceRequest> for AuthMW
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error, Future = ServiceFuture>,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMWService<S>;
    type Future = Ready<Result<Self::Transform, ()>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMWService {
            url: self.auth_service_address.clone(),
            service,
        }))
    }
}

pub struct AuthMWService<S> {
    url: String,
    service: S,
}

async fn verify_token(address: String, token: String) -> Result<(), Error> {
    let resp = reqwest::Client::new()
        .get(format!("http://{}/tokens/{}/verify", address, token))
        .send()
        .await
        .map_err(ErrorInternalServerError)?;
    let status = resp.status();
    if status != StatusCode::OK {
        match resp.text().await {
            Ok(text) => return Err(ErrorInternalServerError(text)),
            Err(e) => return Err(ErrorInternalServerError(e)),
        }
    }
    Ok(())
}

impl<S> Service<ServiceRequest> for AuthMWService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error, Future = ServiceFuture>,
{
    type Error = Error;
    type Response = ServiceResponse;
    type Future = ServiceFuture;

    fn poll_ready(&self, _: &mut core::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let http_req = req.request().clone();
        if let Some(hv) = req.headers().get("X-Auth-Token") {
            if let Ok(token) = hv.to_str() {
                let url = self.url.clone();
                let token = token.to_owned();
                let next = self.service.call(req);
                return Box::pin(async move {
                    match verify_token(url, token).await {
                        Ok(()) => next.await,
                        Err(e) => Ok(ServiceResponse::new(
                            http_req,
                            HttpResponse::new(StatusCode::UNAUTHORIZED)
                                .set_body(BoxBody::new(e.to_string())),
                        )),
                    }
                });
            }
        }
        Box::pin(ready(Ok(ServiceResponse::new(
            http_req,
            HttpResponse::new(StatusCode::UNAUTHORIZED),
        ))))
    }
}
