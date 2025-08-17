use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Releases {
    pub status_code: String,
    pub gameversions: Vec<Version>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    tagid: i64,
    name: String,
    color: String,
}
