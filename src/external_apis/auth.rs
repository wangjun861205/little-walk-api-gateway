use actix_web::{error::ErrorInternalServerError, Error};
use reqwest::{Method, Url};
use serde::Deserialize;
use std::fmt::Display;

use super::common::request;

#[derive(Debug, Deserialize)]
pub struct GenTokenResult {
    pub token: String,
}

pub async fn gen_token(host: impl Display, phone: impl Display) -> Result<String, Error> {
    let url = Url::parse(&format!("http://{}/phones/{}/codes", host, phone))
        .map_err(ErrorInternalServerError)?;
    let res: GenTokenResult = request(Method::PUT, url).await?;
    Ok(res.token)
}
