use crate::core::{
    auth_client::AuthClient, error::Error, requests::DogQuery,
    sms_verification_code_client::SMSVerificationCodeClient,
    upload_client::UploadClient,
};
use bytes::Bytes;
use futures::Stream;
use reqwest::StatusCode;
use std::pin::Pin;

use super::{dog_client::DogClient, requests::DogUpdate};

pub type ByteStream =
    Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send + Sync>>;

pub struct Service<A, U, S, D>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    auth_client: A,
    upload_client: U,
    sms_verification_code_client: S,
    dog_client: D,
}

impl<A, U, S, D> Service<A, U, S, D>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    pub fn new(
        auth_client: A,
        upload_client: U,
        sms_verification_code_client: S,
        dog_client: D,
    ) -> Self {
        Self {
            auth_client,
            upload_client,
            sms_verification_code_client,
            dog_client,
        }
    }

    pub async fn signup(
        &self,
        phone: &str,
        password: &str,
        verification_code: &str,
    ) -> Result<ByteStream, Error> {
        let is_valid = self
            .sms_verification_code_client
            .verify_code(phone, verification_code)
            .await?;
        if !is_valid {
            return Err(Error::InvalidVerificationCode);
        }
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

    pub async fn verify_auth_token(
        &self,
        token: &str,
    ) -> Result<Option<String>, Error> {
        self.auth_client.verify_token(token).await
    }

    pub async fn add_dog(
        &self,
        owner_id: &str,
        dog: ByteStream,
    ) -> Result<ByteStream, Error> {
        self.dog_client.add_dog(owner_id, dog).await
    }

    pub async fn send_verification_code(
        &self,
        phone: &str,
    ) -> Result<ByteStream, Error> {
        self.sms_verification_code_client.send_code(phone).await
    }

    pub async fn upload(
        &self,
        content_type_header: &str,
        user_id: &str,
        size_limit: usize,
        payload: ByteStream,
    ) -> Result<ByteStream, Error> {
        self.upload_client
            .upload(content_type_header, user_id, size_limit, payload)
            .await
    }

    pub async fn download(&self, id: &str) -> Result<ByteStream, Error> {
        self.upload_client.download(id).await
    }

    pub async fn my_dogs(
        &self,
        uid: &str,
        page: i32,
        size: i32,
    ) -> Result<(ByteStream, StatusCode), Error> {
        self.dog_client
            .query_dogs(
                &DogQuery {
                    owner_id_eq: Some(uid.to_owned()),
                },
                page,
                size,
            )
            .await
    }

    pub async fn update_dog_portrait(
        &self,
        uid: &str,
        dog_id: &str,
        portrait_id: &str,
    ) -> Result<(ByteStream, StatusCode), Error> {
        let is_owner = self.dog_client.is_owner_of_the_dog(uid, dog_id).await?;
        if !is_owner {
            return Err(Error::Forbidden);
        }
        self.dog_client
            .update_dog_portrait(dog_id, portrait_id)
            .await
    }

    pub async fn dog_breeds(
        &self,
        category: &str,
    ) -> Result<ByteStream, Error> {
        self.dog_client.query_breeds(category).await
    }
}
