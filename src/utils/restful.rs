use crate::core::error::Error;
use crate::core::service::ByteStream;
use actix_web::HttpRequest;
use futures::TryStreamExt;
use reqwest::{
    header::HeaderMap, multipart::Form, Body, Client, IntoUrl, Method,
    RequestBuilder, StatusCode,
};
use serde::Serialize;
use std::time::Duration;

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
    let resp = builder
        .send()
        .await
        .map_err(|e| Error::NetworkFailure(e.to_string()))?;
    if !resp.status().is_success() {
        let reason = resp
            .text()
            .await
            .map_err(|e| Error::NetworkFailure(e.to_string()))?;
        return Err(Error::AuthServiceError(reason));
    }
    Ok(Box::pin(
        resp.bytes_stream()
            .map_err(|e| Error::NetworkFailure(e.to_string())),
    ))
}

pub async fn request(
    builder: RequestBuilder,
) -> Result<(ByteStream, StatusCode), Error> {
    let resp = builder
        .send()
        .await
        .map_err(|e| Error::NetworkFailure(e.to_string()))?;
    // if !resp.status().is_success() {
    //     let reason = resp
    //         .text()
    //         .await
    //         .map_err(|e| Error::NetworkFailure(e.to_string()))?;
    //     return Err(Error::AuthServiceError(reason));
    // }
    let status_code = resp.status();
    Ok((
        Box::pin(
            resp.bytes_stream()
                .map_err(|e| Error::NetworkFailure(e.to_string())),
        ),
        status_code,
    ))
}

pub fn extract_user_id(req: &HttpRequest) -> Result<&str, Error> {
    let user_id = req
        .headers()
        .get("X-User-ID")
        .ok_or(Error::NoUserID)?
        .to_str()
        .map_err(|e| Error::InvalidUserID(e.to_string()))?;
    Ok(user_id)
}
