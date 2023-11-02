use anyhow::Error;
use bytes::Bytes;

pub trait AuthClient {
    async fn signup(&self, phone: &str, password: &str) -> Result<Bytes, Error>;
    async fn login(&self, phone: &str, password: &str) -> Result<Bytes, Error>;
    async fn verify_token(&self, token: &str) -> Result<String, Error>;
    async fn exists_user(&self, phone: &str) -> Result<bool, Error>;
    async fn generate_token(&self, phone: &str) -> Result<Bytes, Error>;
}
