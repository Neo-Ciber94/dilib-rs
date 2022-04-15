mod api;
mod entities;
mod middlewares;
mod repositories;
mod services;
mod utils;

use crate::entities::todo_task::TodoTask;
use crate::middlewares::audit_logger;
use crate::repositories::{in_memory::InMemoryRepository, Repository};
use crate::services::audit_log_service::AuditLogService;
use actix_web::middleware;
use actix_web::{web, App, HttpServer};
use dilib::add_scoped_trait;
use dilib::global::init_container;
use entities::audit_log::AuditLog;
use uuid::Uuid;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_dependency_injection();
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

fn init_dependency_injection() {
    init_container(|container| {
        // Scoped
        add_scoped_trait!(container, Repository<TodoTask, Uuid> => InMemoryRepository::default())
            .unwrap();
        add_scoped_trait!(container, Repository<AuditLog, Uuid> => InMemoryRepository::default())
            .unwrap();
    })
    .unwrap();
}
