use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json, Query},
    Error,
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

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NearbyRequestsParams {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(flatten)]
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
