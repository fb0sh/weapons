use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub id: String,
    pub name: String,
}
