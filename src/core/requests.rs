use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct DogQuery {
    pub owner_id_eq: Option<String>,
}
