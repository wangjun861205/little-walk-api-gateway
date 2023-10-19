use actix_web::{http::StatusCode, HttpResponse};
use std::fmt::{Debug, Display};

use actix_web::ResponseError;

pub struct Error {
    pub status_code: StatusCode,
    pub message: Box<dyn Display>,
    pub cause: Option<Box<dyn Display>>,
}

impl Error {
    pub fn new(status_code: StatusCode, message: impl Display + 'static) -> Self {
        Self {
            status_code,
            message: Box::new(message),
            cause: None,
        }
    }

    pub fn wrap(
        status_code: StatusCode,
        message: impl Display + 'static,
        cause: impl Display + 'static,
    ) -> Self {
        Self {
            status_code,
            message: Box::new(message),
            cause: Some(Box::new(cause)),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(c) = &self.cause {
            return write!(f, "{}: {}", self.message, c);
        }
        write!(f, "{}", self.message)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> reqwest::StatusCode {
        self.status_code
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
