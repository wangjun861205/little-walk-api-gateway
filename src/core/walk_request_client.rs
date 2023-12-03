use crate::core::dog_client::UpstreamDog as Dog;
use crate::core::error::Error;
use chrono::{DateTime, Utc};
use nb_to_query::{ToQuery, ToQueryDerive};
use serde::{Deserialize, Serialize};

use super::common::Pagination;

#[derive(Debug, Deserialize)]
pub struct UpstreamDog {
    pub id: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpstreamWalkRequest {
    pub id: String,
    pub dogs: Vec<UpstreamDog>,
    pub should_start_after: Option<DateTime<Utc>>,
    pub should_start_before: Option<DateTime<Utc>>,
    pub should_end_after: Option<DateTime<Utc>>,
    pub should_end_before: Option<DateTime<Utc>>,
    pub latitude: f64,
    pub longitude: f64,
    pub distance: Option<f64>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub accepted_by: Option<String>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: String,
    pub acceptances: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct WalkRequest {
    pub id: String,
    pub dogs: Vec<Dog>,
    pub should_start_after: Option<DateTime<Utc>>,
    pub should_start_before: Option<DateTime<Utc>>,
    pub should_end_after: Option<DateTime<Utc>>,
    pub should_end_before: Option<DateTime<Utc>>,
    pub latitude: f64,
    pub longitude: f64,
    pub distance: Option<f64>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub accepted_by: Option<String>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: String,
    pub acceptances: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToQueryDerive)]
pub struct Nearby {
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
}

#[derive(Debug, Serialize, Deserialize, Default, ToQueryDerive)]
pub struct WalkRequestQuery {
    pub id: Option<String>,
    pub dog_ids_includes_all: Option<Vec<String>>,
    pub dog_ids_includes_any: Option<Vec<String>>,
    pub nearby: Option<Nearby>,
    pub accepted_by: Option<String>,
    pub accepted_by_neq: Option<String>,
    pub accepted_by_is_null: bool,
    pub acceptances_includes_all: Option<Vec<String>>,
    pub acceptances_includes_any: Option<Vec<String>>,
    pub pagination: Option<Pagination>,
}

pub trait WalkRequestClient {
    async fn query_walk_requests(
        &self,
        query: WalkRequestQuery,
    ) -> Result<Vec<UpstreamWalkRequest>, Error>;
}
