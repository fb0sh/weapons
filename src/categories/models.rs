use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
pub const CATEGORIES: &str = "categories";

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: RecordId,
    pub name: String,
    pub maintainer: Option<RecordId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
