use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dilib::Container;
use std::sync::Arc;

pub type SharedContainer = web::Data<Container<'static>>;

#[get("/")]
async fn hello(container: SharedContainer) -> impl Responder {
    let hello = container.get_scoped::<String>().unwrap();
    HttpResponse::Ok().body(hello)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(8080);

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(create_container()))
            .service(hello)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

fn create_container() -> Container<'static> {
    let mut container = Container::new();
    container.add_scoped(|| String::from("Hello"));

    container
}
