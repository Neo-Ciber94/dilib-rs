use std::cell::RefCell;
use actix_web::HttpRequest;
use uuid::Uuid;
use dilib::{Singleton, Injectable, Container, get_singleton_trait, get_scoped_trait};
use crate::{Log, Logger, Repository};
use crate::entities::log::HttpVerb;
use crate::services::logger::LogLevel;

pub struct LoggerService {
    logger: Singleton<Box<dyn Logger + Send + Sync>>,
    repository: RefCell<Box<dyn Repository<Log, Uuid>>>
}

impl LoggerService {
    pub async fn log<S: Into<String>>(&self, message: S, level: LogLevel, req: HttpRequest) {
        use actix_web::http::Method;

        let message = message.into().clone();
        self.logger.log(message.clone(), level);

        let ip = req.peer_addr().map(|addr| addr.ip());
        let route = req.match_info().as_str().to_owned();
        let method = match *req.method() {
            Method::GET => HttpVerb::GET,
            Method::POST => HttpVerb::POST,
            Method::PUT => HttpVerb::PUT,
            Method::DELETE => HttpVerb::DELETE,
            _ => HttpVerb::UNKNOWN
        };

        self.repository.borrow_mut().add(Log {
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            ip,
            route,
            method,
            message,
            level,
        }).await;
    }
}

impl Injectable for LoggerService {
    fn resolve(container: &Container) -> Self {
        let logger = get_singleton_trait!(container, Logger).unwrap();
        let repository = get_scoped_trait!(container, Repository<Log, Uuid>).unwrap();

        LoggerService {
            logger,
            repository: RefCell::new(repository)
        }
    }
}