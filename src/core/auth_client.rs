use anyhow::Error;

pub trait AuthClient {
    async fn signup(&self, phone: &str, password: &str) -> Result<String, Error>;
    async fn login(&self, phone: &str, password: &str) -> Result<String, Error>;
    async fn verify_token(&self, token: &str) -> Result<String, Error>;
    async fn exists_user(&self, phone: &str) -> Result<bool, Error>;
}
