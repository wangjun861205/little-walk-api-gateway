use crate::{
    core::{
        error::Error, service::ByteStream,
        sms_verification_code_client::SMSVerificationCodeClient as ISMSVerificationCodeClient,
    },
    utils::{
        io::stream_to_bytes,
        restful::{make_request, parse_url},
    },
};
use http::StatusCode;
use reqwest::Method;
use serde::Deserialize;

pub struct SMSVerificationCodeClient {
    host_and_port: String,
}

impl SMSVerificationCodeClient {
    pub fn new(host_and_port: &str) -> Self {
        Self {
            host_and_port: host_and_port.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct VerifyCodeResp {
    is_ok: bool,
}

impl ISMSVerificationCodeClient for SMSVerificationCodeClient {
    async fn send_code(
        &self,
        phone: &str,
    ) -> Result<ByteStream, crate::core::error::Error> {
        let url = parse_url(
            &&self.host_and_port,
            format!("/phones/{}/codes", phone).as_str(),
            None,
        )?;
        make_request(
            Method::PUT,
            url,
            None,
            Option::<String>::None,
            Option::<String>::None,
            None,
        )
        .await
    }

    async fn verify_code(
        &self,
        phone: &str,
        code: &str,
    ) -> Result<bool, crate::core::error::Error> {
        let url = parse_url(
            &self.host_and_port,
            format!("/phones/{}/codes/{}/verification", phone, code).as_str(),
            None,
        )?;
        let stream = make_request(
            Method::PUT,
            url,
            None,
            Option::<String>::None,
            Option::<String>::None,
            None,
        )
        .await?;
        let bs = stream_to_bytes(stream).await?;
        let result: VerifyCodeResp =
            serde_json::from_slice(&bs).map_err(|e| {
                Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
            })?;
        Ok(result.is_ok)
    }
}
