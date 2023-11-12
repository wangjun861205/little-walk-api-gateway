use crate::core::error::Error;
use actix_web::{body::BoxBody, HttpResponse, ResponseError};

impl ResponseError for Error {
    fn error_response(
        &self,
    ) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .body(BoxBody::new(self.cause.to_owned()))
    }
}
