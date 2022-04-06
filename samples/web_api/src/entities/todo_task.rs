use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::repository::Entity;

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