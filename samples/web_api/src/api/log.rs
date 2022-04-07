use crate::entities::audit_log::AuditLog;
use crate::Repository;
use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse, Responder};
use dilib::{get_scoped_trait, Container};
use uuid::Uuid;

type SharedContainer = Data<Container<'static>>;

#[get("")]
pub async fn get_all(container: SharedContainer) -> impl Responder {
    let repository = get_scoped_trait!(container, Repository<AuditLog, Uuid>).unwrap();
    let result = repository.get_all().await;
    HttpResponse::Ok().json(result)
}

#[get("/{id}")]
pub async fn get_by_id(id: Path<Uuid>, container: SharedContainer) -> impl Responder {
    let id = id.into_inner();
    let repository = get_scoped_trait!(container, Repository<AuditLog, Uuid>).unwrap();
    match repository.get(id).await {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}
