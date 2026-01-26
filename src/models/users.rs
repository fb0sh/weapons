use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
pub const USERS: &str = "users";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: RecordId,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
