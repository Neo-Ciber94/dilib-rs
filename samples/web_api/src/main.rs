mod api;
mod entities;
mod repository;
mod services;

use crate::entities::todo_task::TodoTask;
use crate::repository::{in_memory::InMemoryRepository, Repository};
use crate::services::logger_service::LoggerService;
use crate::services::{console_logger::ConsoleLogger, logger::Logger};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpServer};
use dilib::{register_scoped_trait, register_singleton_trait, Container};
use entities::log::Log;
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
            .wrap(NormalizePath::new(TrailingSlash::Always))
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
    register_scoped_trait!(container, Repository<Log, Uuid>, InMemoryRepository::default());
    container.add_deps::<LoggerService>();

    // Singletons
    register_singleton_trait!(container, Logger, ConsoleLogger);

    container
}
