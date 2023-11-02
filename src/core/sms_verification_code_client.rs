pub trait SMSVerificationCodeClient {
    async fn send_code(&self, phone: &str) -> Result<(), String>;
    async fn verify_code(&self, phone: &str, code: &str) -> Result<(), String>;
}
