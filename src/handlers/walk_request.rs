use actix_web::{error::ErrorInternalServerError, Error};

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

pub struct NearbyRequestsParams {
    pub lat: f64,
    pub lng: f64,
    pub page: i32,
    pub size: i32,
}

pub(crate) struct WalkRequestHandler<A, U, S, D, R>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
    R: walk_request::WalkRequestClient,
{
    service: Service<A, U, S, D, R>,
}

impl<A, U, S, D, R> WalkRequestHandler<A, U, S, D, R>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
    R: walk_request::WalkRequestClient,
{
    pub(crate) fn new(service: Service<A, U, S, D, R>) -> Self {
        Self { service }
    }

    pub(crate) async fn nearby_requests(
        &self,
        params: NearbyRequestsParams,
    ) -> Result<Vec<WalkRequest>, Error> {
        self.service
            .nearby_requests(
                params.lng,
                params.lat,
                20.0,
                Pagination {
                    page: params.page,
                    size: params.size,
                },
            )
            .await
            .map_err(ErrorInternalServerError)
    }
}
