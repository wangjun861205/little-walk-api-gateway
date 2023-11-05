use crate::core::error::Error;

use super::service::ByteStream;

pub trait SMSVerificationCodeClient {
    async fn send_code(&self, phone: &str) -> Result<ByteStream, Error>;
    async fn verify_code(&self, phone: &str, code: &str)
        -> Result<bool, Error>;
}
