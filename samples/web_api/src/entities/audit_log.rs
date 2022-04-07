use crate::repositories::Entity;
use chrono::{DateTime, MIN_DATETIME, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use actix_web::http::Method;
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
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "DEBUG")]
    Debug,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "WARN")]
    Warn,
    #[serde(rename = "ERROR")]
    Error
}

impl From<Method> for HttpVerb {
    fn from(method: Method) -> Self {
        match method {
            Method::GET => HttpVerb::GET,
            Method::POST => HttpVerb::POST,
            Method::PUT => HttpVerb::PUT,
            Method::PATCH => HttpVerb::PATCH,
            Method::DELETE => HttpVerb::DELETE,
            _ => HttpVerb::UNKNOWN,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    id: Uuid,
    message: Option<String>,
    method: HttpVerb,
    route: String,
    level: LogLevel,
    ip: Option<IpAddr>,
    duration_ms: u128,
    user_agent: Option<String>,
    created_at: DateTime<Utc>,
}

impl AuditLog {
    pub fn builder() -> AuditLogBuilder {
        AuditLogBuilder::new()
    }
}

pub struct AuditLogBuilder {
    inner: AuditLog,
}

impl AuditLogBuilder {
    pub fn new() -> AuditLogBuilder {
        AuditLogBuilder {
            inner: AuditLog {
                id: Uuid::default(),
                message: None,
                method: HttpVerb::UNKNOWN,
                route: String::new(),
                level: LogLevel::Info,
                ip: None,
                duration_ms: 0,
                user_agent: None,
                created_at: MIN_DATETIME,
            },
        }
    }

    pub fn message<S: Into<String>>(mut self, message: Option<S>) -> AuditLogBuilder {
        self.inner.message = message.map(|s| s.into());
        self
    }

    pub fn method(mut self, method: HttpVerb) -> AuditLogBuilder {
        self.inner.method = method;
        self
    }

    pub fn route<S: Into<String>>(mut self, route: S) -> AuditLogBuilder {
        self.inner.route = route.into();
        self
    }

    pub fn level(mut self, level: LogLevel) -> AuditLogBuilder {
        self.inner.level = level;
        self
    }

    pub fn ip(mut self, ip: Option<IpAddr>) -> AuditLogBuilder {
        self.inner.ip = ip;
        self
    }

    pub fn duration_ms(mut self, duration_ms: u128) -> AuditLogBuilder {
        self.inner.duration_ms = duration_ms;
        self
    }

    pub fn user_agent<S: Into<String>>(mut self, user_agent: Option<S>) -> AuditLogBuilder {
        self.inner.user_agent = user_agent.map(|s| s.into());
        self
    }

    pub fn build(mut self) -> AuditLog {
        self.inner.created_at = Utc::now();
        self.inner.id = Uuid::new_v4();
        self.inner
    }
}

impl Entity<Uuid> for AuditLog {
    fn id(&self) -> &Uuid {
        &self.id
    }
}
