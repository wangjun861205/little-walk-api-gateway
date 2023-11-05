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
    InvalidRequestHeader(String),
    InvalidVerificationCode,
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
            Self::InvalidRequestHeader(cause) => {
                write!(f, "Invalid request header: {}", cause)
            }
            Self::InvalidVerificationCode => {
                write!(f, "Invalid verification code")
            }
        }
    }
}

impl StdError for Error {}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::InvalidURL(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::InvalidResponse(value.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderName> for Error {
    fn from(value: reqwest::header::InvalidHeaderName) -> Self {
        Self::InvalidRequestHeader(value.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(value: reqwest::header::InvalidHeaderValue) -> Self {
        Self::InvalidRequestHeader(value.to_string())
    }
}
