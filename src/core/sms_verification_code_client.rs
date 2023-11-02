use anyhow::Error;

pub trait SMSVerificationCodeClient {
    async fn send_code(&self, phone: &str) -> Result<(), Error>;
    async fn verify_code(&self, phone: &str, code: &str) -> Result<bool, Error>;
}
