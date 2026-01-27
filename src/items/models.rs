use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;


#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: RecordId,
    pub maintainer: Option<RecordId>,
    pub category: Option<RecordId>,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<RecordId>>,

    pub format: Option<String>,
    pub content_length: Option<i32>,
    pub ref_url: Option<String>,
    pub source: Option<String>,
    pub authors: Option<Vec<String>>,
    pub status: Option<String>,     // draft/done/verified
    pub read_scope: Option<String>, // public/private/restricted

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
