use crate::repository::Entity;
use crate::services::logger::LogLevel;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use uuid::Uuid;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpVerb {
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub id: Uuid,
    pub message: String,
    pub method: HttpVerb,
    pub route: String,
    pub level: LogLevel,
    pub ip: Option<IpAddr>,
    pub created_at: DateTime<Utc>,
}

impl Entity<Uuid> for Log {
    fn id(&self) -> &Uuid {
        &self.id
    }
}
