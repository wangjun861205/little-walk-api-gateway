use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub size: i32,
}
