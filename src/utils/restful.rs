use crate::core::error::Error;
use crate::core::service::ByteStream;
use actix_web::error::ErrorBadRequest;
use actix_web::{FromRequest, HttpRequest};
use futures::{future, TryStreamExt};
use http::StatusCode;
use nb_serde_query::from_str;
use nb_serde_query::to_string as to_query;
use reqwest::{
    header::HeaderMap, multipart::Form, Body, Client, IntoUrl, Method,
    RequestBuilder,
};
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;
use url::Url;

pub(crate) async fn make_request<U, P, B>(
    method: Method,
    url: U,
    headers: Option<HeaderMap>,
    params: Option<P>,
    body: Option<B>,
    multipart: Option<Form>,
) -> Result<ByteStream, Error>
where
    U: IntoUrl,
    P: Serialize,
    B: Into<Body>,
{
    let mut builder = Client::new()
        .request(method, url)
        .timeout(Duration::from_secs(10));
    if let Some(headers) = headers {
        builder = builder.headers(headers);
    }
    if let Some(params) = params {
        builder = builder.query(&params);
    }
    if let Some(body) = body {
        builder = builder
            .header("Content-Type", "application/json")
            .body(body);
    }
    if let Some(multipart) = multipart {
        builder = builder.multipart(multipart);
    }
    let resp = builder.send().await.map_err(|e| {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
    })?;
    if !resp.status().is_success() {
        let status_code = resp.status();
        let reason = resp.text().await.map_err(|e| {
            Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
        })?;
        return Err(Error::new(status_code.as_u16(), reason));
    }
    Ok(Box::pin(resp.bytes_stream().map_err(|e| {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
    })))
}

pub async fn request(builder: RequestBuilder) -> Result<ByteStream, Error> {
    let resp = builder.send().await.map_err(|e| {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
    })?;
    if !resp.status().is_success() {
        let status_code = resp.status();
        let reason = resp.text().await.map_err(|e| {
            Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
        })?;
        return Err(Error::new(status_code.as_u16(), reason));
    }
    Ok(Box::pin(resp.bytes_stream().map_err(|e| {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
    })))
}

pub fn extract_user_id(req: &HttpRequest) -> Result<&str, Error> {
    let user_id = req
        .headers()
        .get("X-User-ID")
        .ok_or(Error::new(StatusCode::UNAUTHORIZED.as_u16(), "no user id"))?
        .to_str()
        .map_err(|e| Error::new(StatusCode::UNAUTHORIZED.as_u16(), e))?;
    Ok(user_id)
}

pub fn to_query_string<T: Serialize>(
    params: &T,
) -> Result<Option<String>, Error> {
    let s = to_query(params)
        .map_err(|e| Error::new(StatusCode::BAD_REQUEST.as_u16(), e))?;
    if s.is_empty() {
        return Ok(None);
    }
    Ok(Some(s))
}

pub fn parse_url(
    host_and_port: &str,
    path: &str,
    params: Option<&str>,
) -> Result<Url, Error> {
    let mut url =
        Url::parse(&format!("http://{}", host_and_port)).map_err(|e| {
            Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
        })?;
    url.set_path(path);
    url.set_query(params);
    Ok(url)
}

pub struct Query<T>(pub T);

impl<T> FromRequest for Query<T>
where
    for<'de> T: Deserialize<'de>,
{
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let query = req.query_string();
        match from_str(query)
            .map_err(|e| Error::new(StatusCode::BAD_REQUEST.as_u16(), e))
        {
            Ok(v) => futures::future::ready(Ok(Query(v))),
            Err(e) => futures::future::ready(Err(e)),
        }
    }
}
