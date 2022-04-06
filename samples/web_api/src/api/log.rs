use actix_web::web::{Data, Path};
use actix_web::{ get, HttpResponse, Responder};
use uuid::Uuid;
use dilib::{Container, get_scoped_trait};
use crate::entities::log::Log;
use crate::Repository;

type SharedContainer = Data<Container<'static>>;

#[get("/")]
pub async fn get_all(container: SharedContainer) -> impl Responder {
    let repository = get_scoped_trait!(container, Repository<Log, Uuid>).unwrap();
    let result = repository.get_all().await;
    HttpResponse::Ok().json(result)
}

#[get("/{id}")]
pub async fn get_by_id(id: Path<Uuid>, container: SharedContainer) -> impl Responder {
    let id = id.into_inner();
    let repository = get_scoped_trait!(container, Repository<Log, Uuid>).unwrap();
    match repository.get(id).await {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}