use crate::core::error::Error;
use crate::core::service::Service;
use crate::core::sms_verification_code_client::SMSVerificationCodeClient;
use crate::core::upload_client::UploadClient;
use crate::core::{auth_client::AuthClient, dog_client::DogClient};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginBySMSVerificationCodeParams {
    pub phone: String,
    pub code: String,
}

#[utoipa::path(
    put,
    path = "/login/by_sms_verification_code",
    responses(
        (status = 200, description = "successfully login"),
        (status = 403, description = "failed to login")
    ),
    params(
        ("phone" = String, Query, description = "phone number"),
        ("code" = String, Query, description = "verification code")
    )
)]
pub async fn login_by_sms_verification_code<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    Json(LoginBySMSVerificationCodeParams { phone, code }): Json<
        LoginBySMSVerificationCodeParams,
    >,
) -> Result<HttpResponse, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    let stream = service
        .login_by_sms_verification_code(&phone, &code)
        .await?;
    Ok(HttpResponse::Ok().streaming(stream))
}

#[derive(Debug, Deserialize)]
pub struct LoginByPasswordParams {
    pub phone: String,
    pub password: String,
}

pub async fn login_by_password<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    Json(LoginByPasswordParams { phone, password }): Json<
        LoginByPasswordParams,
    >,
) -> Result<HttpResponse, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    Ok(HttpResponse::Ok()
        .streaming(service.login_by_password(&phone, &password).await?))
}

#[derive(Debug, Deserialize)]
pub struct SignupParams {
    pub phone: String,
    pub password: String,
    pub verification_code: String,
}

pub async fn signup<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    Json(SignupParams {
        phone,
        password,
        verification_code,
    }): Json<SignupParams>,
) -> Result<HttpResponse, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    let res = service
        .signup(&phone, &password, &verification_code)
        .await?;
    Ok(HttpResponse::Ok().streaming(res))
}

pub async fn send_verification_code<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    phone: Path<(String,)>,
) -> Result<HttpResponse, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    let res = service.send_verification_code(&phone.as_ref().0).await?;
    Ok(HttpResponse::Ok().streaming(res))
}

#[derive(Debug, Serialize)]
pub struct VerifyAuthTokenResp {
    id: String,
}

pub async fn verify_auth_token<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    token: Path<(String,)>,
) -> Result<Json<VerifyAuthTokenResp>, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    let id = service.verify_auth_token(&token.as_ref().0).await?;
    Ok(Json(VerifyAuthTokenResp { id }))
}
