use crate::core::error::Error;
use crate::core::requests::DogUpdate;
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

    async fn is_owner_of_the_dog(
        &self,
        owner_id: &str,
        dog_id: &str,
    ) -> Result<bool, Error>;

    async fn update_dog_portrait(
        &self,
        dog_id: &str,
        portrait_id: &str,
    ) -> Result<(ByteStream, StatusCode), Error>;

    async fn query_breeds(&self, category: &str) -> Result<ByteStream, Error>;
}
