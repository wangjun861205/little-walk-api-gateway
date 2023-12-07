use std::{pin::Pin, process::Output};

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    web::{Data, Json, Query},
    Error, FromRequest,
};

use crate::core::{
    clients::{
        auth::AuthClient, dog::DogClient,
        sms_verification_code::SMSVerificationCodeClient, upload::UploadClient,
        walk_request,
    },
    common::Pagination,
    entities::WalkRequest,
    service::Service,
};
use std::future::Future;

use nb_serde_query::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NearbyRequestsParams {
    pub latitude: f64,
    pub longitude: f64,
    pub pagination: Pagination,
}

impl FromRequest for NearbyRequestsParams {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let query_str = req.query_string().to_owned();
        Box::pin(async move { from_str(&query_str).map_err(ErrorBadRequest) })
    }
}

pub(crate) async fn nearby_requests<A, U, S, D, R>(
    service: Data<Service<A, U, S, D, R>>,
    params: NearbyRequestsParams,
) -> Result<Json<Vec<WalkRequest>>, Error>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
    R: walk_request::WalkRequestClient,
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
