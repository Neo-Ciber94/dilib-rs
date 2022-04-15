use crate::entities::todo_task::TodoTask;
use crate::Repository;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, post, put, HttpRequest, HttpResponse, Responder};
use dilib::get_scoped;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TodoTaskCreate {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TodoTaskUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[get("")]
pub async fn get_all(_req: HttpRequest) -> impl Responder {
    let repository = get_scoped!(trait Repository<TodoTask, Uuid>).unwrap();
    let result = repository.get_all().await;
    HttpResponse::Ok().json(result)
}

#[get("/{id}")]
pub async fn get_by_id(
    id: Path<Uuid>,
    _req: HttpRequest,
) -> impl Responder {
    let id = id.into_inner();
    let repository = get_scoped!(trait Repository<TodoTask, Uuid>).unwrap();
    match repository.get(id).await {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
pub async fn create(
    data: Json<TodoTaskCreate>,
    _req: HttpRequest,
) -> impl Responder {
    let mut repository = get_scoped!(trait Repository<TodoTask, Uuid>).unwrap();
    let data = data.into_inner();

    let new_todo = TodoTask {
        id: Uuid::new_v4(),
        title: data.title,
        content: data.description,
        completed_at: None,
    };

    let result = repository.add(new_todo).await;
    HttpResponse::Ok().json(result)
}

#[put("/{id}")]
pub async fn update(
    id: Path<Uuid>,
    data: Json<TodoTaskUpdate>,
    _req: HttpRequest,
) -> impl Responder {
    let id = id.into_inner();
    let data = data.into_inner();
    let mut repository = get_scoped!(trait Repository<TodoTask, Uuid>).unwrap();

    if let Some(mut to_update) = repository.get(id).await {
        to_update.title = data.title.unwrap_or(to_update.title);
        to_update.content = to_update.content.or(data.content);

        if let Some(task) = repository.update(to_update).await {
            return HttpResponse::Ok().json(task);
        }
    }

    HttpResponse::NotFound().finish()
}

#[delete("/{id}")]
pub async fn delete(
    id: Path<Uuid>,
    _req: HttpRequest,
) -> impl Responder {
    let id = id.into_inner();
    let mut repository = get_scoped!(trait Repository<TodoTask, Uuid>).unwrap();

    match repository.delete(id).await {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/complete/{id}")]
pub async fn complete(
    id: Path<Uuid>,
    _req: HttpRequest,
) -> impl Responder {
    let id = id.into_inner();
    let mut repository = get_scoped!(trait Repository<TodoTask, Uuid>).unwrap();

    if let Some(mut task) = repository.get(id).await {
        if task.completed_at.is_some() {
            return HttpResponse::Ok().json(task);
        }

        task.completed_at = Some(chrono::Utc::now());

        if let Some(updated) = repository.update(task).await {
            return HttpResponse::Ok().json(updated);
        }
    };

    HttpResponse::NotFound().finish()
}

#[post("/toggle/{id}")]
pub async fn toggle(
    id: Path<Uuid>,
    _req: HttpRequest,
) -> impl Responder {
    let id = id.into_inner();
    let mut repository = get_scoped!(trait Repository<TodoTask, Uuid>).unwrap();

    if let Some(mut task) = repository.get(id).await {
        if task.completed_at.is_some() {
            task.completed_at = None;
        } else {
            task.completed_at = Some(chrono::Utc::now());
        }

        if let Some(updated) = repository.update(task).await {
            return HttpResponse::Ok().json(updated);
        }
    }

    HttpResponse::NotFound().finish()
}
