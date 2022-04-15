use crate::{AuditLog, Repository};
use dilib::{get_scoped_trait, provide, Container, Inject};
use futures_util::lock::Mutex;
use uuid::Uuid;

#[provide]
pub struct AuditLogService {
    repository: Mutex<Box<dyn Repository<AuditLog, Uuid> + Send + Sync>>,
}

#[allow(dead_code)]
impl AuditLogService {
    pub async fn log(&self, data: AuditLog) {
        self.repository.lock().await.add(data).await;
    }
}

impl Inject for AuditLogService {
    fn inject(container: &Container) -> Self {
        let repository = get_scoped_trait!(container, Repository<AuditLog, Uuid>).unwrap();

        AuditLogService {
            repository: Mutex::new(repository),
        }
    }
}
