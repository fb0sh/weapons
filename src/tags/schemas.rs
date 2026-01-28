use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TagInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTag {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTag {
    pub name: Option<String>,
}
