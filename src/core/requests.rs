use super::common::Pagination;
use chrono::{DateTime, Utc};
use little_walk_dog::core::repository::BreedQuery;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize)]
pub struct DogQuery {
    pub id: Option<String>,
    pub id_in: Option<Vec<String>>,
    pub owner_id: Option<String>,
    pub pagination: Option<Pagination>,
}

pub struct DogUpdate {
    pub portrait_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DogPortraitUpdate {
    pub portrait_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DogCreateIncome {
    pub name: String,
    pub gender: String,
    pub breed: BreedQuery,       // 品种
    pub birthday: DateTime<Utc>, // 生日
    // pub is_sterilized: bool,     // 是否绝育
    // pub introduction: String,
    pub tags: Vec<String>,
    pub portrait_id: Option<String>,
}
