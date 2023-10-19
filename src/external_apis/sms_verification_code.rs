use crate::external_apis::common::request;
use crate::ServiceAddresses;
use actix_web::{error::ErrorInternalServerError, web::Data, Error};
use reqwest::{Method, Url};

pub async fn verify_code(
    service_address: String,
    phone: String,
    code: String,
) -> Result<(), Error> {
    let url = Url::parse(&format!(
        "http://{}/phones/{}/codes/{}",
        &service_address, &phone, &code
    ))
    .map_err(ErrorInternalServerError)?;
    request::<()>(Method::GET, url).await
}

pub async fn send_code(service_address: String, phone: String) -> Result<(), Error> {
    let url = Url::parse(&format!(
        "http://{}/phones/{}/codes",
        &service_address, &phone
    ))
    .map_err(ErrorInternalServerError)?;
    request::<()>(Method::PUT, url).await
}
