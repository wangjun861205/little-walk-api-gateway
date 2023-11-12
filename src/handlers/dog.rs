use actix_web::{
    web::{Bytes, Data, Json, Path, Payload, Query},
    HttpRequest, HttpResponse,
};
use http::StatusCode;

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
        .ok_or(Error::new(StatusCode::UNAUTHORIZED, "no X-User-ID"))?
        .to_str()
        .map_err(|e| Error::new(StatusCode::BAD_REQUEST, e))?;
    let body: Vec<Result<Bytes, Error>> = payload
        .map_err(|e| Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))
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
        .ok_or(Error::new(StatusCode::FORBIDDEN, "no X-User-ID"))?
        .to_str()
        .map_err(|e| Error::new(StatusCode::BAD_REQUEST, e))?;
    let content_type_header = req
        .headers()
        .get("Content-Type")
        .ok_or(Error::new(
            StatusCode::BAD_REQUEST,
            "Content-Type header is required",
        ))?
        .to_str()
        .map_err(|e| Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))?;
    let body: Vec<Result<Bytes, Error>> = payload
        .map_err(|e| Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))
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
    let stream = service.my_dogs(uid, page.page, page.size).await?;
    Ok(HttpResponse::Ok()
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
    let stream = service
        .update_dog_portrait(uid, &dog_id.as_ref().0, &portrait_id)
        .await?;
    Ok(HttpResponse::Ok()
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

pub async fn update_dog<A, U, S, D>(
    service: Data<Service<A, U, S, D>>,
    dog_id: Path<(String,)>,
    req: HttpRequest,
    body: Bytes,
) -> Result<HttpResponse, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    let uid = extract_user_id(&req)?;
    let dog_id = &dog_id.as_ref().0;
    let stream = service.update_dog(uid, dog_id, body).await?;
    Ok(HttpResponse::Ok().streaming(stream))
}
