use crate::core::error::Error;
use bytes::Bytes;

use super::service::ByteStream;

pub trait DogClient {
    async fn add_dog(
        &self,
        owner_id: &str,
        body: ByteStream,
    ) -> Result<ByteStream, Error>;
    async fn dogs_by_owner_id(
        &self,
        owner_id: &str,
    ) -> Result<ByteStream, Error>;
}
