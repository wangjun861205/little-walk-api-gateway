use std::io::ErrorKind;

use crate::core::error::Error;
use crate::core::service::ByteStream;
use crate::utils::io::stream_to_bytes;
use crate::utils::restful::{make_request, parse_url, RequestBody};
use crate::{
    core::clients::auth::AuthClient as IAuthClient, utils::restful::request,
};
use futures::io::BufReader;
use futures::{Stream, StreamExt, TryStreamExt};
use http::StatusCode;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, StreamDeserializer};
use std::io::Read;

#[derive(Clone)]
pub struct AuthClient {
    host_and_port: String,
}

impl AuthClient {
    pub fn new(host_and_port: &str) -> Self {
        Self {
            host_and_port: host_and_port.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ExistsUserResp {
    exists: bool,
}

#[derive(Debug, Serialize)]
pub struct LoginReq {
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignupReq {
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenResp {
    id: String,
}

impl IAuthClient for AuthClient {
    async fn exists_user(&self, phone: &str) -> Result<bool, Error> {
        let body = make_request(
            Method::GET,
            &self.host_and_port,
            &format!("/phones/{}/exists", phone),
            None,
            Option::<()>::None,
            RequestBody::<()>::None,
        )
        .await?;
        let bs = stream_to_bytes(body).await?;
        let result: ExistsUserResp = from_slice(&bs).map_err(|e| {
            Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
        })?;
        Ok(result.exists)
    }

    async fn generate_token(&self, phone: &str) -> Result<ByteStream, Error> {
        make_request(
            Method::PUT,
            &self.host_and_port,
            &format!("/phones/{}/tokens", phone),
            None,
            Option::<()>::None,
            RequestBody::<()>::None,
        )
        .await
    }

    async fn login(
        &self,
        phone: &str,
        password: &str,
    ) -> Result<ByteStream, Error> {
        make_request(
            Method::PUT,
            &self.host_and_port,
            "/login",
            None,
            Option::<()>::None,
            RequestBody::Json(LoginReq {
                phone: phone.into(),
                password: password.into(),
            }),
        )
        .await
    }

    async fn signup(
        &self,
        phone: &str,
        password: &str,
    ) -> Result<ByteStream, Error> {
        make_request(
            Method::POST,
            &self.host_and_port,
            "/signup",
            None,
            Option::<()>::None,
            RequestBody::Json(SignupReq {
                phone: phone.into(),
                password: password.into(),
            }),
        )
        .await
    }

    async fn verify_token(&self, token: &str) -> Result<String, Error> {
        let url = parse_url(
            &self.host_and_port,
            &format!("/tokens/{}/verification", token),
            None,
        )?;
        let builder = Client::new().request(Method::GET, url);
        let stream = request(builder).await?;
        let bs = stream_to_bytes(stream).await?;
        let result: VerifyTokenResp =
            serde_json::from_slice(&bs).map_err(|e| {
                Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
            })?;
        Ok(result.id)
    }
}
