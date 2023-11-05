use actix_web::{
    web::{Bytes, Data, Path, Payload},
    HttpRequest, HttpResponse,
};

use crate::core::{
    auth_client::AuthClient, dog_client::DogClient, error::Error,
    service::Service, sms_verification_code_client::SMSVerificationCodeClient,
    upload_client::UploadClient,
};

use futures::{stream, StreamExt, TryStreamExt};

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
