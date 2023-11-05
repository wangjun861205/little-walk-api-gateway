use crate::{
    core::{
        service::ByteStream,
        sms_verification_code_client::SMSVerificationCodeClient as ISMSVerificationCodeClient,
    },
    utils::{io::stream_to_bytes, restful::make_request},
};
use reqwest::Method;
use serde::Deserialize;
use url::Url;

pub struct SMSVerificationCodeClient {
    base_url: Url,
}

impl SMSVerificationCodeClient {
    pub fn new(base_url: Url) -> Self {
        Self { base_url }
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
        let url = self
            .base_url
            .join(format!("/phones/{}/codes", phone).as_str())?;
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
        let url = self.base_url.join(
            format!("/phones/{}/codes/{}/verification", phone, code).as_str(),
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
        let result: VerifyCodeResp = serde_json::from_slice(&bs)?;
        Ok(result.is_ok)
    }
}
