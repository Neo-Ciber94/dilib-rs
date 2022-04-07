use crate::{AuditLog, Repository};
use dilib::{get_scoped_trait, Container, Injectable};
use std::cell::RefCell;
use uuid::Uuid;

pub struct AuditLogService {
    repository: RefCell<Box<dyn Repository<AuditLog, Uuid>>>,
}

#[allow(dead_code)]
impl AuditLogService {
    pub async fn log(&self, data: AuditLog) {
        self.repository.borrow_mut().add(data).await;
    }
}

impl Injectable for AuditLogService {
    fn resolve(container: &Container) -> Self {
        let repository = get_scoped_trait!(container, Repository<AuditLog, Uuid>).unwrap();

        AuditLogService {
            repository: RefCell::new(repository),
        }
    }
}
