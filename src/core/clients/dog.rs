use bytes::Bytes;

use crate::core::error::Error;
use chrono::{DateTime, Utc};

use crate::core::{requests::DogQuery, service::ByteStream};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Breed {
    pub id: String,
    pub category: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dog {
    pub id: String,
    pub name: String,
    pub gender: String,
    pub breed: Breed,            // 品种
    pub birthday: DateTime<Utc>, // 生日
    pub is_sterilized: bool,     // 是否绝育
    pub introduction: String,
    pub owner_id: String,
    pub tags: Vec<String>,
    pub portrait_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub trait DogClient: Clone + 'static {
    async fn add_dog(
        &self,
        owner_id: &str,
        body: ByteStream,
    ) -> Result<ByteStream, Error>;
    async fn dogs_by_owner_id(
        &self,
        owner_id: &str,
    ) -> Result<ByteStream, Error>;

    async fn query_dogs(&self, query: &DogQuery) -> Result<Vec<Dog>, Error>;

    async fn is_owner_of_the_dog(
        &self,
        owner_id: &str,
        dog_id: &str,
    ) -> Result<bool, Error>;

    async fn update_dog_portrait(
        &self,
        dog_id: &str,
        portrait_id: &str,
    ) -> Result<ByteStream, Error>;

    async fn query_breeds(&self, category: &str) -> Result<ByteStream, Error>;

    async fn update_dog(
        &self,
        dog_id: &str,
        body: Bytes,
    ) -> Result<ByteStream, Error>;
}
