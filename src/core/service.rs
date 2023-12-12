use crate::core::{
    clients::{
        auth::AuthClient, dog,
        sms_verification_code::SMSVerificationCodeClient, upload::UploadClient,
        walk_request,
    },
    entities::WalkRequest,
    error::Error,
    requests::DogQuery,
};
use actix_web::web::Data;
use bytes::Bytes;
use futures::{future::try_join_all, Future, Stream};
use reqwest::StatusCode;
use std::pin::Pin;

use super::{
    clients::{dog::DogClient, walk_request::WalkRequestClient},
    common::Pagination,
};

pub type ByteStream =
    Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send + Sync>>;

#[derive(Clone)]
pub struct Service<A, U, S, D, R>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: dog::DogClient,
    R: WalkRequestClient,
{
    auth_client: A,
    upload_client: U,
    sms_verification_code_client: S,
    dog_client: D,
    walk_request_client: R,
}

impl<A, U, S, D, R> Service<A, U, S, D, R>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: dog::DogClient,
    R: WalkRequestClient,
{
    pub fn new(
        auth_client: A,
        upload_client: U,
        sms_verification_code_client: S,
        dog_client: D,
        walk_request_client: R,
    ) -> Self {
        Self {
            auth_client,
            upload_client,
            sms_verification_code_client,
            dog_client,
            walk_request_client,
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
            return Err(Error::new(
                StatusCode::BAD_REQUEST.as_u16(),
                "invalid sms verification code",
            ));
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
            return Err(Error::new(
                StatusCode::NOT_FOUND.as_u16(),
                "user not exists",
            ));
        }
        let ok = self
            .sms_verification_code_client
            .verify_code(phone, verification_code)
            .await?;
        if !ok {
            return Err(Error::new(
                StatusCode::BAD_REQUEST.as_u16(),
                "invalid sms verification code",
            ));
        }
        self.auth_client.generate_token(phone).await
    }

    pub async fn verify_auth_token(
        &self,
        token: &str,
    ) -> Result<String, Error> {
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
    ) -> Result<Vec<dog::Dog>, Error> {
        self.dog_client
            .query_dogs(&DogQuery {
                owner_id: Some(uid.to_owned()),
                pagination: Some(Pagination { page, size }),
                ..Default::default()
            })
            .await
    }

    pub async fn update_dog_portrait(
        &self,
        uid: &str,
        dog_id: &str,
        portrait_id: &str,
    ) -> Result<ByteStream, Error> {
        let is_owner = self.dog_client.is_owner_of_the_dog(uid, dog_id).await?;
        if !is_owner {
            return Err(Error::new(
                StatusCode::FORBIDDEN.as_u16(),
                "no permission",
            ));
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

    pub async fn update_dog(
        &self,
        uid: &str,
        dog_id: &str,
        req_body: Bytes,
    ) -> Result<ByteStream, Error> {
        let is_owner = self.dog_client.is_owner_of_the_dog(uid, dog_id).await?;
        if !is_owner {
            return Err(Error::new(
                StatusCode::FORBIDDEN.as_u16(),
                "no permission",
            ));
        }
        self.dog_client.update_dog(dog_id, req_body).await
    }

    pub async fn nearby_requests(
        &self,
        longitude: f64,
        latitude: f64,
        radius: f64,
        pagination: Pagination,
    ) -> Result<Vec<WalkRequest>, Error> {
        let fs = self
            .walk_request_client
            .query_walk_requests(walk_request::WalkRequestQuery {
                nearby: Some(walk_request::Nearby {
                    latitude,
                    longitude,
                    radius,
                }),
                pagination: Some(pagination),
                ..Default::default()
            })
            .await?
            .into_iter()
            .map(|r| async move {
                let dogs = self
                    .dog_client
                    .query_dogs(&DogQuery {
                        id_in: Some(r.dog_ids.clone()),
                        ..Default::default()
                    })
                    .await?;
                Ok(WalkRequest::from((r, dogs)))
            })
            .collect::<Vec<_>>();
        try_join_all(fs).await
    }

    pub(crate) fn no_op_processor(
        &self,
    ) -> impl FnOnce(
        Bytes,
    ) -> Pin<
        Box<dyn Future<Output = Result<Bytes, Error>> + 'static>,
    > + Clone
           + 'static {
        |bytes| Box::pin(async move { Ok(bytes) })
    }

    pub(crate) fn fill_dogs_processor(
        &self,
    ) -> impl FnOnce(
        Bytes,
    ) -> Pin<
        Box<dyn Future<Output = Result<Bytes, Error>> + 'static>,
    > + Clone
           + 'static {
        let service = self.clone();
        move |bytes: Bytes| {
            Box::pin(async move {
                let req: crate::core::clients::walk_request::WalkRequest =
                    serde_json::from_slice(&bytes).map_err(Error::wrap(
                        StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    ))?;
                let dogs = service
                    .dog_client
                    .query_dogs(&DogQuery {
                        id_in: Some(req.dog_ids.clone()),
                        ..Default::default()
                    })
                    .await?;
                let res = WalkRequest::from((req, dogs));
                Ok(serde_json::to_vec(&res)
                    .map_err(Error::wrap(
                        StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    ))?
                    .into())
            })
        }
    }
}
