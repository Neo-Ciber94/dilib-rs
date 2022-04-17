extern crate core;

mod api;
mod entities;
mod middlewares;
mod repositories;
mod services;
mod utils;

use crate::middlewares::audit_logger;
use crate::services::AuditLogService;
use actix_web::middleware;
use actix_web::{web, App, HttpServer};
use dilib::add_scoped_trait;
use dilib::global::init_container;
use entities::{AuditLog, TodoTask};
use uuid::Uuid;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_dependency_injection().await;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(8080);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/todos")
                    .wrap(audit_logger::<TodoTask>())
                    .service(api::todo_task::get_all)
                    .service(api::todo_task::get_by_id)
                    .service(api::todo_task::create)
                    .service(api::todo_task::update)
                    .service(api::todo_task::delete)
                    .service(api::todo_task::complete)
                    .service(api::todo_task::toggle),
            )
            .service(
                web::scope("/api/logs")
                    .service(api::log::get_all)
                    .service(api::log::get_by_id),
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[allow(dead_code)]
async fn init_dependency_injection() {
    use crate::repositories::{InMemoryRepository, StorageRepository, Repository};

    // Used for the sake of the example
    enum RepositoryType {
        InMemory,
        Storage,
    }

    const REPOSITORY_TYPE: RepositoryType = RepositoryType::InMemory;

    init_container(|container| {
        // Scoped
        match REPOSITORY_TYPE {
            RepositoryType::InMemory => {
                add_scoped_trait!(container, Repository<TodoTask, Uuid> => InMemoryRepository::default())
                    .unwrap();
                add_scoped_trait!(container, Repository<AuditLog, Uuid> => InMemoryRepository::default())
                    .unwrap();
            }
            RepositoryType::Storage => {
                add_scoped_trait!(container, Repository<TodoTask, Uuid> => StorageRepository::new("todo_tasks"))
                    .unwrap();
                add_scoped_trait!(container, Repository<AuditLog, Uuid> => StorageRepository::new("audit_logs"))
                    .unwrap();
            }
        }
    })
    .unwrap();
}
