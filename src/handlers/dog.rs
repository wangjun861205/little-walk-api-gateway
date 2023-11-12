use actix_web::{
    web::{Bytes, Data, Json, Path, Payload, Query},
    HttpRequest, HttpResponse,
};

use crate::{
    core::{
        auth_client::AuthClient, dog_client::DogClient, error::Error,
        requests::DogPortraitUpdate, service::Service,
        sms_verification_code_client::SMSVerificationCodeClient,
        upload_client::UploadClient,
    },
    utils::restful::extract_user_id,
};

use futures::{stream, StreamExt, TryStreamExt};
use serde::Deserialize;

use super::common::Pagination;

pub async fn add_dog<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    req: HttpRequest,
    payload: Payload,
) -> Result<HttpResponse, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    let owner_id = req
        .headers()
        .get("X-User-ID")
        .ok_or(Error::InvalidRequestHeader("X-User-ID".into()))?
        .to_str()
        .map_err(|e| Error::InvalidRequestHeader(e.to_string()))?;
    let body: Vec<Result<Bytes, Error>> = payload
        .map_err(|e| Error::NetworkFailure(e.to_string()))
        .collect()
        .await;
    let body = Box::pin(stream::iter(body.into_iter()));
    let result = service.add_dog(owner_id, body).await?;
    Ok(HttpResponse::Ok().streaming(result))
}

pub async fn upload_portrait<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    req: HttpRequest,
    payload: Payload,
) -> Result<HttpResponse, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    let user_id = req
        .headers()
        .get("X-User-ID")
        .ok_or(Error::InvalidRequestHeader("X-User-ID".into()))?
        .to_str()
        .map_err(|e| Error::InvalidRequestHeader(e.to_string()))?;
    let content_type_header = req
        .headers()
        .get("Content-Type")
        .ok_or(Error::InvalidRequestHeader(
            "Content-Type header is required".into(),
        ))?
        .to_str()
        .map_err(|e| Error::InvalidRequestHeader(e.to_string()))?;
    let body: Vec<Result<Bytes, Error>> = payload
        .map_err(|e| Error::NetworkFailure(e.to_string()))
        .collect()
        .await;
    let body = Box::pin(stream::iter(body.into_iter()));
    let result = service
        .upload(content_type_header, user_id, 1024 * 1024, body)
        .await?;
    Ok(HttpResponse::Ok().streaming(result))
}

pub async fn download_portrait<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    id: Path<(String,)>,
) -> Result<HttpResponse, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    let result = service.download(&id.as_ref().0).await?;
    Ok(HttpResponse::Ok().streaming(result))
}

pub async fn my_dogs<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    req: HttpRequest,
    Query(page): Query<Pagination>,
) -> Result<HttpResponse, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    let uid = extract_user_id(&req)?;
    let (stream, status) = service.my_dogs(uid, page.page, page.size).await?;
    Ok(HttpResponse::build(status)
        .insert_header(("Content-Type", "application/json; charset=utf-8"))
        .streaming(stream))
}

#[derive(Debug, Deserialize)]
pub struct UpdateDogPortraitReq {
    portrait_id: String,
}

pub async fn update_dog_portrait<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    req: HttpRequest,
    Json(UpdateDogPortraitReq { portrait_id }): Json<UpdateDogPortraitReq>,
    dog_id: Path<(String,)>,
) -> Result<HttpResponse, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    let uid = extract_user_id(&req)?;
    let (stream, status) = service
        .update_dog_portrait(uid, &dog_id.as_ref().0, &portrait_id)
        .await?;
    Ok(HttpResponse::build(status)
        .insert_header(("Content-Type", "application/json; charset=utf-8"))
        .streaming(stream))
}

#[derive(Debug, Deserialize)]
pub struct BreedQuery {
    category_eq: String,
}

pub async fn query_breeds<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    Query(BreedQuery { category_eq }): Query<BreedQuery>,
) -> Result<HttpResponse, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    let stream = service.dog_breeds(&category_eq).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json; charset=utf-8"))
        .streaming(stream))
}
