use anyhow::Error;
use chrono::{DateTime, Local};

pub trait DogClient {
    async fn add_dog(&self, owner_id: &str, name: &str, breed: &str, gender: &str) -> Result<String, Error>;
    async fn update_birthday(&self, birthday: DateTime<Local>) -> Result<(), Error>;
}
