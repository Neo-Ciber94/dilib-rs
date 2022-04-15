use actix_web::web::Path;
use actix_web::{get, HttpResponse, Responder};
use uuid::Uuid;
use crate::Repository;
use crate::entities::audit_log::AuditLog;
use dilib::resolve;

#[get("")]
pub async fn get_all() -> impl Responder {
    let repository = resolve!(trait Repository<AuditLog, Uuid>).unwrap();
    let mut result = repository.get_all().await;
    result.sort_by_key(|x| std::cmp::Reverse(x.created_at().clone()));
    HttpResponse::Ok().json(result)
}

#[get("/{id}")]
pub async fn get_by_id(id: Path<Uuid>) -> impl Responder {
    let id = id.into_inner();
    let repository = resolve!(trait Repository<AuditLog, Uuid>).unwrap();
    match repository.get(id).await {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}
