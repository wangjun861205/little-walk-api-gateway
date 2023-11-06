use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub size: i32,
}
