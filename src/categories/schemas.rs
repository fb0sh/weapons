use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub id: String,
    pub name: String,
    pub you_are_maintainer: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub maintainer: Option<RecordId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub maintainer: Option<RecordId>,
}
