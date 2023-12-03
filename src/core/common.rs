use nb_to_query::{ToQuery, ToQueryDerive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToQueryDerive)]
pub struct Pagination {
    pub page: i32,
    pub size: i32,
}
