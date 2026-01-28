use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::_core::SurrealModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<RecordId>,
    pub role: String, // maintainer/manager
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SurrealModel for User {
    fn table() -> &'static str {
        "users"
    }

    fn id(&self) -> Option<&RecordId> {
        self.id.as_ref()
    }
}
