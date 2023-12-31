use crate::core::error::Error;

use crate::core::service::ByteStream;

pub trait AuthClient: Clone + 'static {
    async fn signup(
        &self,
        phone: &str,
        password: &str,
    ) -> Result<ByteStream, Error>;
    async fn login(
        &self,
        phone: &str,
        password: &str,
    ) -> Result<ByteStream, Error>;
    async fn verify_token(&self, token: &str) -> Result<String, Error>;
    async fn exists_user(&self, phone: &str) -> Result<bool, Error>;
    async fn generate_token(&self, phone: &str) -> Result<ByteStream, Error>;
}
