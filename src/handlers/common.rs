use std::pin::Pin;

use actix_web::{web::Bytes, Handler, HttpRequest, HttpResponse};
use futures::Future;
use http::StatusCode;
use reqwest::{header::HeaderMap, Client, Method};
use serde::Deserialize;
use url::Url;

use crate::{
    core::error::Error,
    utils::{io::stream_to_bytes, restful::request},
};

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub size: i32,
}

fn parse_url(
    host_and_port: &str,
    path: &str,
    query: &str,
) -> Result<Url, Error> {
    let base_url =
        Url::parse(&format!("http://{}", host_and_port)).map_err(|e| {
            Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
        })?;
    let mut url = base_url.join(path).map_err(|e| {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
    })?;
    if query.is_empty() {
        url.set_query(Some(query));
    }
    Ok(url)
}

pub(crate) fn pass_through<QP, RP, QF, RF>(
    host_and_port: &str,
    path: Option<&str>,
    request_body_processor: QP,
    response_processor: RP,
) -> impl Handler<(HttpRequest, Bytes), Output = Result<HttpResponse, Error>>
where
    QP: FnOnce(&HttpRequest, Bytes) -> QF,
    QP: Clone + 'static,
    RP: FnOnce(Bytes) -> RF,
    RP: Clone + 'static,
    QF: Future<Output = Result<Bytes, Error>> + 'static,
    RF: Future<Output = Result<Bytes, Error>> + 'static,
{
    let host_and_port = host_and_port.to_owned();
    let path = path.map(|p| p.to_owned());
    move |req: HttpRequest,
          bytes: Bytes|
          -> Pin<Box<dyn Future<Output = Result<HttpResponse, Error>>>> {
        let response_processor = response_processor.clone();
        let host_and_port = host_and_port.clone();
        let path = if let Some(path) = path.clone() {
            path.clone()
        } else {
            req.path().to_owned()
        };
        let mut headers = HeaderMap::new();
        for (name, value) in req.headers() {
            headers.insert(name, value.clone());
        }
        match parse_url(&host_and_port, &path, req.query_string()) {
            Ok(mut url) => {
                url.set_query(if req.query_string() != "" {
                    Some(req.query_string())
                } else {
                    None
                });
                let mut builder;
                match req.method() {
                    &Method::GET | &Method::OPTIONS | &Method::TRACE => {
                        builder = Client::new().get(url)
                    }
                    &Method::POST => builder = Client::new().post(url),
                    &Method::PUT => builder = Client::new().put(url),
                    &Method::DELETE => builder = Client::new().delete(url),
                    &Method::HEAD => builder = Client::new().head(url),
                    &Method::PATCH => builder = Client::new().patch(url),
                    _ => {
                        return Box::pin(async {
                            Err(Error::new(
                                StatusCode::METHOD_NOT_ALLOWED.as_u16(),
                                "unsupported method",
                            ))
                        })
                    }
                }
                let request_body_processor = request_body_processor.clone();
                Box::pin(async move {
                    let bytes = request_body_processor(&req, bytes).await?;
                    headers.insert("Content-Length", bytes.len().into());
                    builder = builder.headers(headers).body(bytes);
                    let stream = request(builder).await?;
                    let bytes = stream_to_bytes(stream).await?;
                    let res = response_processor(bytes).await?;
                    Ok(HttpResponse::Ok().body(res))
                })
            }
            Err(e) => Box::pin(async move {
                Err(Error::new(StatusCode::BAD_REQUEST.as_u16(), e.to_string()))
            }),
        }
    }
}
