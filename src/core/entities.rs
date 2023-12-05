use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::clients::{dog, walk_request};

#[derive(Debug, Serialize, Deserialize)]
pub struct Breed {
    pub id: String,
    pub category: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<dog::Breed> for Breed {
    fn from(value: dog::Breed) -> Self {
        Self {
            id: value.id,
            category: value.category,
            name: value.name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
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

impl From<dog::Dog> for Dog {
    fn from(value: dog::Dog) -> Self {
        Self {
            id: value.id,
            name: value.name,
            gender: value.gender,
            breed: Breed::from(value.breed),
            birthday: value.birthday,
            is_sterilized: value.is_sterilized,
            introduction: value.introduction,
            owner_id: value.owner_id,
            tags: value.tags,
            portrait_id: value.portrait_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WalkRequest {
    pub id: String,
    pub dogs: Vec<Dog>,
    pub latitude: f64,
    pub longitude: f64,
    pub distance: Option<f64>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub accepted_by: Option<String>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<(walk_request::WalkRequest, Vec<dog::Dog>)> for WalkRequest {
    fn from((req, dogs): (walk_request::WalkRequest, Vec<dog::Dog>)) -> Self {
        Self {
            id: req.id,
            dogs: dogs.into_iter().map(Dog::from).collect(),
            latitude: req.latitude,
            longitude: req.longitude,
            distance: req.distance,
            canceled_at: req.canceled_at,
            accepted_by: req.accepted_by,
            accepted_at: req.accepted_at,
            started_at: req.started_at,
            finished_at: req.finished_at,
            status: req.status,
            created_at: req.created_at,
            updated_at: req.updated_at,
        }
    }
}
