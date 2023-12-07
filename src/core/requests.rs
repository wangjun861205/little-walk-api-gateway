use serde::Serialize;

use super::common::Pagination;
use nb_serde_query::Array;

#[derive(Debug, Default, Serialize)]
pub struct DogQuery {
    pub id: Option<String>,
    pub id_in: Option<Array<String>>,
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
