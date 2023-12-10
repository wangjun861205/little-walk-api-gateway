use std::pin::Pin;

use actix_web::{web::Bytes, Handler, HttpRequest, HttpResponse};
use bytes::{BufMut, BytesMut};
use futures::{
    Future, FutureExt, Stream, StreamExt, TryFutureExt, TryStreamExt,
};
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

pub(crate) fn pass_through<P, F, B>(
    host_and_port: &str,
    path: Option<&str>,
    response_processor_builder: B,
) -> impl Handler<(HttpRequest, Bytes), Output = Result<HttpResponse, Error>>
where
    B: FnOnce() -> P + Clone,
    P: FnOnce(Bytes) -> F,
    P: Clone + 'static,
    F: Future<Output = Result<Bytes, Error>> + Send,
{
    let host_and_port = host_and_port.to_owned();
    let path = path.map(|p| p.to_owned());
    let response_processor = response_processor_builder.clone()();
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
                builder = builder.headers(headers).body(bytes);
                Box::pin(async move {
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
