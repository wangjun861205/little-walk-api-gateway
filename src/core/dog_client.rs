use crate::core::error::Error;
use reqwest::StatusCode;

use super::{requests::DogQuery, service::ByteStream};

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

    async fn query_dogs(
        &self,
        query: &DogQuery,
        page: i32,
        size: i32,
    ) -> Result<(ByteStream, StatusCode), Error>;
}
