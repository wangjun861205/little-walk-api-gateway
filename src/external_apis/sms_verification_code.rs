use crate::external_apis::common::request;
use actix_web::{error::ErrorInternalServerError, Error};
use reqwest::{Method, Url};
use std::fmt::Display;

pub async fn verify_code(
    service_address: impl Display,
    phone: impl Display,
    code: impl Display,
) -> Result<(), Error> {
    let url = Url::parse(&format!(
        "http://{}/phones/{}/codes/{}",
        &service_address, &phone, &code
    ))
    .map_err(ErrorInternalServerError)?;
    request::<()>(Method::GET, url).await
}

pub async fn send_code(service_address: impl Display, phone: impl Display) -> Result<(), Error> {
    let url = Url::parse(&format!(
        "http://{}/phones/{}/codes",
        &service_address, &phone
    ))
    .map_err(ErrorInternalServerError)?;
    request::<()>(Method::PUT, url).await
}
