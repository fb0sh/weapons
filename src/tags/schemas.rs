use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct TagInfo {
    pub id: String,
    pub name: String,
    pub you_are_maintainer: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTag {
    pub name: String,
    pub maintainer: Option<RecordId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTag {
    pub name: Option<String>,
    pub maintainer: Option<RecordId>,
}
