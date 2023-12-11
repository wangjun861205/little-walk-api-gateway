use std::pin::Pin;

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    web::{Data, Json},
    Error, FromRequest,
};

use bytes::Bytes;
use futures::Future;
use http::StatusCode;

use crate::{core::requests::DogQuery, utils::restful::Query};

use crate::core::{
    clients::{
        auth::AuthClient, dog::DogClient,
        sms_verification_code::SMSVerificationCodeClient, upload::UploadClient,
        walk_request::WalkRequestClient,
    },
    common::Pagination,
    entities::WalkRequest,
    error::Error as CoreError,
    service::Service,
};

use serde::{Deserialize, Serialize};

pub(crate) fn fill_dogs<A, U, S, D, R>(
    service: Data<Service<A, U, S, D, R>>,
) -> impl FnOnce(
    Bytes,
) -> Pin<
    Box<dyn Future<Output = Result<Bytes, CoreError>> + 'static>,
> + Clone
where
    A: AuthClient + 'static,
    U: UploadClient + 'static,
    S: SMSVerificationCodeClient + 'static,
    D: DogClient + 'static,
    R: WalkRequestClient + 'static,
{
    |bytes: Bytes| {
        Box::pin(async move {
            let req: crate::core::clients::walk_request::WalkRequest =
                serde_json::from_slice(&bytes).map_err(CoreError::wrap(
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                ))?;
            let dogs = service
                .query_dogs(&DogQuery {
                    id_in: Some(req.dog_ids.clone()),
                    ..Default::default()
                })
                .await?;
            let res = WalkRequest::from((req, dogs));
            Ok(serde_json::to_vec(&res)
                .map_err(CoreError::wrap(
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                ))?
                .into())
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearbyRequestsParams {
    pub latitude: f64,
    pub longitude: f64,
    pub pagination: Pagination,
}

pub(crate) async fn nearby_requests<A, U, S, D, R>(
    service: Data<Service<A, U, S, D, R>>,
    Query(params): Query<NearbyRequestsParams>,
) -> Result<Json<Vec<WalkRequest>>, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
    R: WalkRequestClient,
{
    Ok(Json(
        service
            .nearby_requests(
                params.longitude,
                params.latitude,
                20.0,
                Pagination {
                    page: params.pagination.page,
                    size: params.pagination.size,
                },
            )
            .await
            .map_err(ErrorInternalServerError)?,
    ))
}
