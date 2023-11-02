use anyhow::Error;
use bytes::Bytes;

pub trait DogClient {
    async fn add_dog(&self, owner_id: &str, body: Bytes) -> Result<String, Error>;
    async fn dogs_by_owner_id(&self, owner_id: &str) -> Result<Bytes, Error>;
}
