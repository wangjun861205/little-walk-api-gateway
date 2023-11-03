use crate::core::auth_client::AuthClient as IAuthClient;
use crate::core::error::Error;
use crate::core::service::ByteStream;
use crate::utils::io::stream_to_bytes;
use crate::utils::restful::make_request;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use url::Url;

pub struct AuthClient {
    base_url: Url,
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
    id: Option<String>,
}

impl IAuthClient for AuthClient {
    async fn exists_user(&self, phone: &str) -> Result<bool, Error> {
        let url = self
            .base_url
            .join(&format!("/users/{}/exists", phone))
            .map_err(|e| Error::InvalidURL(e.to_string()))?;
        let body = make_request(
            Method::GET,
            url,
            Option::<(String, String)>::None,
            Option::<Vec<u8>>::None,
            None,
        )
        .await?;
        let bs = stream_to_bytes(body).await?;
        let result: ExistsUserResp = from_slice(&bs)
            .map_err(|e| Error::InvalidResponse(e.to_string()))?;
        Ok(result.exists)
    }

    async fn generate_token(&self, phone: &str) -> Result<ByteStream, Error> {
        let url = self
            .base_url
            .join(&format!("/phones/{}/tokens", phone))
            .map_err(|e| Error::InvalidURL(e.to_string()))?;
        make_request(
            Method::PUT,
            url,
            Option::<(String, String)>::None,
            Option::<Vec<u8>>::None,
            None,
        )
        .await
    }

    async fn login(
        &self,
        phone: &str,
        password: &str,
    ) -> Result<ByteStream, Error> {
        let url = self
            .base_url
            .join("/login")
            .map_err(|e| Error::InvalidURL(e.to_string()))?;
        make_request(
            Method::PUT,
            url,
            Option::<(String, String)>::None,
            Some(LoginReq {
                phone: phone.into(),
                password: password.into(),
            }),
            None,
        )
        .await
    }

    async fn signup(
        &self,
        phone: &str,
        password: &str,
    ) -> Result<ByteStream, Error> {
        let url = self
            .base_url
            .join("/signup")
            .map_err(|e| Error::InvalidURL(e.to_string()))?;
        make_request(
            Method::PUT,
            url,
            Option::<(String, String)>::None,
            Some(SignupReq {
                phone: phone.into(),
                password: password.into(),
            }),
            None,
        )
        .await
    }

    async fn verify_token(&self, token: &str) -> Result<Option<String>, Error> {
        let url = self
            .base_url
            .join(&format!("/tokens/{}/verify", token))
            .map_err(|e| Error::InvalidURL(e.to_string()))?;
        let stream = make_request(
            Method::PUT,
            url,
            Option::<(String, String)>::None,
            Option::<Vec<u8>>::None,
            None,
        )
        .await?;
        let bs = stream_to_bytes(stream).await?;
        let result: VerifyTokenResp = serde_json::from_slice(&bs)
            .map_err(|e| Error::InvalidResponse(e.to_string()))?;
        Ok(result.id)
    }
}
