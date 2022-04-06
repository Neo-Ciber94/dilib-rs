use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, put, HttpResponse, Responder, HttpRequest};
use uuid::Uuid;
use dilib::{Container, get_scoped_trait};
use serde::{Serialize, Deserialize};
use crate::entities::todo_task::TodoTask;
use crate::{LoggerService, Repository};
use crate::services::logger::LogLevel;

type SharedContainer = Data<Container<'static>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TodoTaskCreate {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TodoTaskUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[get("/")]
pub async fn get_all(container: SharedContainer, req: HttpRequest) -> impl Responder {
    let repository = get_scoped_trait!(container, Repository<TodoTask, Uuid>).unwrap();
    let logger_service = container.get::<LoggerService>().unwrap();

    let result = repository.get_all().await;
    logger_service.log("Get all todo tasks", LogLevel::Info, req).await;
    HttpResponse::Ok().json(result)
}

#[get("/{id}")]
pub async fn get_by_id(id: Path<Uuid>, container: SharedContainer) -> impl Responder {
    let id = id.into_inner();
    let repository = get_scoped_trait!(container, Repository<TodoTask, Uuid>).unwrap();
    match repository.get(id).await {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/")]
pub async fn create(data: Json<TodoTaskCreate>, container: SharedContainer, req: HttpRequest) -> impl Responder {
    let mut repository = get_scoped_trait!(container, Repository<TodoTask, Uuid>).unwrap();
    let logger_service = container.get::<LoggerService>().unwrap();
    let data = data.into_inner();
    let new_todo = TodoTask {
        id: Uuid::new_v4(),
        title: data.title,
        content: data.description,
        completed_at: None
    };

    let result = repository.add(new_todo).await;
    logger_service.log("Create todo task", LogLevel::Info, req).await;
    HttpResponse::Ok().json(result)
}

#[put("/{id}")]
pub async fn update(id: Path<Uuid>, data: Json<TodoTaskUpdate>, container: SharedContainer) -> impl Responder {
    let id = id.into_inner();
    let data = data.into_inner();
    let mut repository = get_scoped_trait!(container, Repository<TodoTask, Uuid>).unwrap();

    if let Some(mut to_update) = repository.get(id).await {
        to_update.title = data.title.unwrap_or(to_update.title);
        to_update.content = data.description.or(to_update.content);

        if let Some(task) = repository.update(to_update).await {
            return HttpResponse::Ok().json(task);
        }
    }

    HttpResponse::NotFound().finish()
}

#[delete("/{id}")]
pub async fn delete(id: Path<Uuid>, container: SharedContainer) -> impl Responder {
    let id = id.into_inner();
    let mut repository = get_scoped_trait!(container, Repository<TodoTask, Uuid>).unwrap();

    match repository.delete(id).await {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{id}/complete")]
pub async fn complete(id: Path<Uuid>, container: SharedContainer) -> impl Responder {
    let id = id.into_inner();
    let mut repository = get_scoped_trait!(container, Repository<TodoTask, Uuid>).unwrap();

    match repository.get(id).await {
        Some(mut task) => {
            if task.completed_at.is_some() {
                return HttpResponse::Ok().json(task);
            }

            task.completed_at = Some(chrono::Utc::now());
            match repository.update(task).await {
                Some(task) => HttpResponse::Ok().json(task),
                None => HttpResponse::NotFound().finish(),
            }
        },
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{id}/toggle")]
pub async fn toggle(id: Path<Uuid>, container: SharedContainer) -> impl Responder {
    let id = id.into_inner();
    let mut repository = get_scoped_trait!(container, Repository<TodoTask, Uuid>).unwrap();

    match repository.get(id).await {
        Some(mut task) => {
            if task.completed_at.is_some() {
                task.completed_at = None;
            } else {
                task.completed_at = Some(chrono::Utc::now());
            }

            match repository.update(task).await {
                Some(task) => HttpResponse::Ok().json(task),
                None => HttpResponse::NotFound().finish(),
            }
        },
        None => HttpResponse::NotFound().finish(),
    }
}