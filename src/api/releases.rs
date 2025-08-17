use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameVersionsResponse {
    pub statuscode: String,
    pub gameversions: Vec<Version>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub tagid: i64,
    pub name: String,
    pub color: String,
}
