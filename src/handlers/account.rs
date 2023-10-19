use crate::{external_apis::sms_verification_code::verify_code, ServiceAddresses};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
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
) -> HttpResponse {
    verify_code(
        service_addresses
            .sms_verification_code_service_address
            .clone(),
        params.phone,
        params.code,
    )
    .await?;
}
