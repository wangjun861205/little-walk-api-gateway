use nb_to_query::{ToQuery, ToQueryDerive};
use serde::Serialize;

use super::common::Pagination;

#[derive(Debug, Default, Serialize, ToQueryDerive)]
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
