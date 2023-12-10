use std::error::Error as StdError;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Error {
    pub status_code: u16,
    pub cause: String,
}

impl Error {
    pub fn new<C>(status_code: u16, cause: C) -> Self
    where
        C: Display,
    {
        Self {
            status_code,
            cause: cause.to_string(),
        }
    }

    pub fn wrap<C>(status_code: u16) -> impl FnOnce(C) -> Self
    where
        C: Display,
    {
        move |c| Error::new(status_code, c)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status: {}, cause: {}", self.status_code, self.cause)
    }
}

impl StdError for Error {}
