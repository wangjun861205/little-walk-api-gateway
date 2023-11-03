use crate::core::{
    auth_client::AuthClient, error::Error,
    sms_verification_code_client::SMSVerificationCodeClient,
    upload_client::UploadClient,
};
use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;

pub type ByteStream =
    Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send + Sync>>;

pub struct Service<A, U, S>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
{
    auth_client: A,
    upload_client: U,
    sms_verification_code_client: S,
}

impl<A, U, S> Service<A, U, S>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
{
    pub fn new(
        auth_client: A,
        upload_client: U,
        sms_verification_code_client: S,
    ) -> Self {
        Self {
            auth_client,
            upload_client,
            sms_verification_code_client,
        }
    }

    pub async fn signup(
        &self,
        phone: &str,
        password: &str,
    ) -> Result<ByteStream, Error> {
        self.auth_client.signup(phone, password).await
    }

    pub async fn login_by_password(
        &self,
        phone: &str,
        password: &str,
    ) -> Result<ByteStream, Error> {
        self.auth_client.login(phone, password).await
    }

    pub async fn login_by_sms_verification_code(
        &self,
        phone: &str,
        verification_code: &str,
    ) -> Result<ByteStream, Error> {
        let exists = self.auth_client.exists_user(phone).await?;
        if !exists {
            return Err(Error::UserNotExists);
        }
        let ok = self
            .sms_verification_code_client
            .verify_code(phone, verification_code)
            .await?;
        if !ok {
            return Err(Error::InvalidToken);
        }
        self.auth_client.generate_token(phone).await
    }
}
