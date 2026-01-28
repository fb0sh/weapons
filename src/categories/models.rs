use crate::_core::SurrealModel;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<RecordId>,
    pub name: String,
    pub maintainer: Option<RecordId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SurrealModel for Category {
    fn table() -> &'static str {
        "categories"
    }

    fn id(&self) -> Option<&RecordId> {
        self.id.as_ref()
    }
}
