use crate::core::error::Error;
use crate::core::service::ByteStream;
use futures::TryStreamExt;
use reqwest::{multipart::Form, Body, Client, IntoUrl, Method};
use serde::Serialize;
use std::time::Duration;

pub(crate) async fn make_request<U, P, B>(
    method: Method,
    url: U,
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
    if let Some(params) = params {
        builder = builder.query(&params);
    }
    if let Some(body) = body {
        builder = builder.body(body);
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
