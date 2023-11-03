use std::error::Error as StdError;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    UserNotExists,
    InvalidToken,
    InvalidURL(String),
    NetworkFailure(String),
    AuthServiceError(String),
    InvalidResponse(String),
    InvalidRequestBody(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserNotExists => write!(f, "User not exists"),
            Self::InvalidToken => write!(f, "Invalid token"),
            Self::InvalidURL(cause) => write!(f, "Invalid URL: {}", cause),
            Self::NetworkFailure(cause) => {
                write!(f, "Network failure: {}", cause)
            }
            Self::AuthServiceError(cause) => {
                write!(f, "Auth service error: {}", cause)
            }
            Self::InvalidResponse(cause) => {
                write!(f, "Invalid response: {}", cause)
            }
            Self::InvalidRequestBody(cause) => {
                write!(f, "Invalid request body: {}", cause)
            }
        }
    }
}

impl StdError for Error {}
