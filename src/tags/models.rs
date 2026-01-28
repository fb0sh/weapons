use crate::_core::{SurrealModel, curd::OwnableResource};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<RecordId>,
    pub name: String,
    pub maintainer: Option<RecordId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SurrealModel for Tag {
    fn table() -> &'static str {
        "tags"
    }

    fn id(&self) -> Option<&RecordId> {
        self.id.as_ref()
    }
}

impl OwnableResource for Tag {
    fn owner_id(&self) -> Option<&RecordId> {
        self.maintainer.as_ref()
    }
}
