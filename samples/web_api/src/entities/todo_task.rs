use crate::repositories::Entity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TodoTask {
    pub id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Entity<Uuid> for TodoTask {
    fn id(&self) -> &Uuid {
        &self.id
    }
}
