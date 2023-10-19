use crate::error::Error;
use actix_web::body::BoxBody;
use actix_web::dev::{Service, Transform};
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse};
use std::future::Future;
use std::pin::Pin;
use std::{
    future::{ready, Ready},
    task::Poll,
};

type ServiceFuture = Pin<Box<dyn Future<Output = Result<HttpResponse, ()>>>>;

pub struct AuthMW {
    url: String,
}

impl<S> Transform<S, HttpRequest> for AuthMW
where
    S: Service<HttpRequest, Response = HttpResponse, Error = (), Future = ServiceFuture>,
{
    type Response = HttpResponse;
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

async fn verify_token(url: String, token: String) -> Result<(), Error> {
    let resp = reqwest::Client::new()
        .get(format!("{}/{}", url, token))
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

impl<S> Service<HttpRequest> for AuthMWService<S>
where
    S: Service<HttpRequest, Response = HttpResponse, Error = (), Future = ServiceFuture>,
{
    type Error = ();
    type Response = HttpResponse;
    type Future = ServiceFuture;

    fn poll_ready(&self, _: &mut core::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: HttpRequest) -> Self::Future {
        if req.path() == "/login" {
            return Box::pin(self.service.call(req));
        }
        if let Some(hv) = req.clone().headers().get("X-Auth-Token") {
            if let Ok(token) = hv.to_str() {
                let url = self.url.clone();
                let token = token.to_owned();
                let next = self.service.call(req);
                return Box::pin(async move {
                    match verify_token(url, token).await {
                        Ok(()) => next.await,
                        Err(e) => {
                            Ok(HttpResponse::new(e.status_code)
                                .set_body(BoxBody::new(e.to_string())))
                        }
                    }
                });
            }
        }
        Box::pin(ready(Ok(HttpResponse::new(StatusCode::UNAUTHORIZED))))
    }
}
