use crate::core::error::Error;
use chrono::{DateTime, Utc};
use nb_serde_query::Array;
use serde::{Deserialize, Serialize};

use crate::core::common::Pagination;

#[derive(Debug, Deserialize, Default)]
pub struct WalkRequest {
    pub id: String,
    pub dog_ids: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Nearby {
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WalkRequestQuery {
    pub id: Option<String>,
    pub dog_ids_includes_all: Option<Array<String>>,
    pub dog_ids_includes_any: Option<Array<String>>,
    #[serde(flatten)]
    pub nearby: Option<Nearby>,
    pub accepted_by: Option<String>,
    pub accepted_by_neq: Option<String>,
    pub accepted_by_is_null: bool,
    pub acceptances_includes_all: Option<Array<String>>,
    pub acceptances_includes_any: Option<Array<String>>,
    #[serde(flatten)]
    pub pagination: Option<Pagination>,
}

pub trait WalkRequestClient {
    async fn query_walk_requests(
        &self,
        query: WalkRequestQuery,
    ) -> Result<Vec<WalkRequest>, Error>;
}
