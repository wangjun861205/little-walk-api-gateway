use crate::core::error::Error;
use actix_web::{body::BoxBody, HttpResponse, ResponseError};

impl ResponseError for Error {
    fn error_response(
        &self,
    ) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            Error::AuthServiceError(cause) => {
                HttpResponse::InternalServerError()
                    .body(BoxBody::new(cause.to_owned()))
            }
            Error::InvalidRequestBody(cause) => {
                HttpResponse::BadRequest().body(BoxBody::new(cause.to_owned()))
            }
            Error::InvalidResponse(cause) => {
                HttpResponse::InternalServerError()
                    .body(BoxBody::new(cause.to_owned()))
            }
            Error::InvalidToken => HttpResponse::Unauthorized()
                .body(BoxBody::new("invalid auth token")),
            Error::InvalidURL(cause) => HttpResponse::MisdirectedRequest()
                .body(BoxBody::new(cause.to_owned())),
            Error::NetworkFailure(cause) => HttpResponse::InternalServerError()
                .body(BoxBody::new(cause.to_owned())),
            Error::UserNotExists => HttpResponse::Unauthorized()
                .body(BoxBody::new("user not exists")),
            Error::InvalidRequestHeader(cause) => {
                HttpResponse::InternalServerError()
                    .body(BoxBody::new(cause.to_owned()))
            }
            Error::InvalidVerificationCode => {
                HttpResponse::Unauthorized().body("invalid verification code")
            }
            Error::NoUserID => HttpResponse::Unauthorized().body("no user id"),
            Error::InvalidUserID(cause) => HttpResponse::Unauthorized()
                .body(format!("invalid user id: {}", cause)),
            Error::Forbidden => HttpResponse::Forbidden().body("forbidden"),
        }
    }
}
