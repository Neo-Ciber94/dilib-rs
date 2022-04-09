use crate::{AuditLog, Repository};
use dilib::{get_scoped_trait, Container, Injectable};
use futures_util::lock::Mutex;
use uuid::Uuid;

pub struct AuditLogService {
    repository: Mutex<Box<dyn Repository<AuditLog, Uuid> + Send + Sync>>,
}

#[allow(dead_code)]
impl AuditLogService {
    pub async fn log(&self, data: AuditLog) {
        self.repository.lock().await.add(data).await;
    }
}

impl Injectable for AuditLogService {
    fn resolve(container: &Container) -> Self {
        let repository = get_scoped_trait!(container, Repository<AuditLog, Uuid>).unwrap();

        AuditLogService {
            repository: Mutex::new(repository),
        }
    }
}
