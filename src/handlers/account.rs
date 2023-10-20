use crate::{
    external_apis::{auth::gen_token, sms_verification_code::verify_code},
    ServiceAddresses,
};
use actix_web::{
    body::BoxBody,
    web::{Data, Query},
    Error, HttpResponse,
};
use reqwest::StatusCode;
use serde::Deserialize;

pub async fn login() -> HttpResponse {
    unimplemented!()
}

pub async fn logout() -> HttpResponse {
    unimplemented!()
}

pub async fn register() -> HttpResponse {
    unimplemented!()
}

#[derive(Debug, Deserialize)]
pub struct LoginBySMSVerificationCodeParams {
    pub phone: String,
    pub code: String,
}

pub async fn login_by_sms_verification_code(
    service_addresses: Data<ServiceAddresses>,
    params: Query<LoginBySMSVerificationCodeParams>,
) -> Result<HttpResponse, Error> {
    verify_code(
        &service_addresses.sms_verification_code_service_address,
        &params.phone,
        &params.code,
    )
    .await?;
    let token = gen_token(&service_addresses.auth_service_address, &params.phone).await?;
    Ok(HttpResponse::new(StatusCode::OK).set_body(BoxBody::new(token)))
}
