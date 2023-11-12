use http::StatusCode;
use std::error::Error as StdError;
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    pub status_code: StatusCode,
    pub cause: String,
}

impl Error {
    pub fn new<C>(status_code: StatusCode, cause: C) -> Self
    where
        C: Display,
    {
        Self {
            status_code,
            cause: cause.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status: {}, cause: {}", self.status_code, self.cause)
    }
}

impl StdError for Error {}
