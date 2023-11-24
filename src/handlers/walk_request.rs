use actix_web::{web::Query, HttpResponse};

use crate::core::{
    auth_client::AuthClient, dog_client::DogClient, service::Service,
    sms_verification_code_client::SMSVerificationCodeClient,
    upload_client::UploadClient,
};

pub struct NearbyRequestsParams {
    pub lat: f64,
    pub lng: f64,
    pub page: i32,
    pub size: i32,
}

pub(crate) struct WalkRequestHandler<A, U, S, D>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    service: Service<A, U, S, D>,
}

impl<A, U, S, D> WalkRequestHandler<A, U, S, D>
where
    A: AuthClient,
    U: UploadClient,
    S: SMSVerificationCodeClient,
    D: DogClient,
{
    pub(crate) fn new(service: Service<A, U, S, D>) -> Self {
        Self { service }
    }
}
