mod api;
mod entities;
mod middlewares;
mod repositories;
mod services;

use crate::entities::todo_task::TodoTask;
use crate::middlewares::AuditLogger;
use crate::repositories::{in_memory::InMemoryRepository, Repository};
use crate::services::audit_log_service::AuditLogService;
use actix_web::middleware;
use actix_web::{web, App, HttpServer};
use dilib::{register_scoped_trait, Container};
use entities::audit_log::AuditLog;
use uuid::Uuid;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(8080);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .wrap(AuditLogger)
            .app_data(web::Data::new(create_container()))
            .service(
                web::scope("/api/todos")
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

fn create_container() -> Container<'static> {
    let mut container = Container::new();

    // Scoped
    register_scoped_trait!(container, Repository<TodoTask, Uuid>, InMemoryRepository::default());
    register_scoped_trait!(container, Repository<AuditLog, Uuid>, InMemoryRepository::default());
    container.add_deps::<AuditLogService>();

    // Singletons

    container
}
