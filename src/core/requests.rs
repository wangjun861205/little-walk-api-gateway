use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct DogQuery {
    pub owner_id_eq: Option<String>,
}

pub struct DogUpdate {
    pub portrait_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DogPortraitUpdate {
    pub portrait_id: String,
}
