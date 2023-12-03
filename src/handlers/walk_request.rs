use actix_web::{error::ErrorInternalServerError, Error};

use crate::core::{
    auth_client::AuthClient,
    common::Pagination,
    dog_client::DogClient,
    service::Service,
    sms_verification_code_client::SMSVerificationCodeClient,
    upload_client::UploadClient,
    walk_request_client::{WalkRequest, WalkRequestClient},
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
    R: WalkRequestClient,
{
    service: Service<A, U, S, D, R>,
}

impl<A, U, S, D, R> WalkRequestHandler<A, U, S, D, R>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
    R: WalkRequestClient,
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
