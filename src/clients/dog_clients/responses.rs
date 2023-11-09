use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IsOwnerOfTheDogResp {
    pub is_owner: bool,
}
