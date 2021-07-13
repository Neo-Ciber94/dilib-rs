use std::any::Any;
use std::sync::Arc;
use crate::scoped::Scoped;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ProviderKind {
    Scoped,
    Singleton
}

#[derive(Clone)]
pub enum Provider {
    Scoped(Scoped),
    Singleton(Arc<dyn Any + Send + Sync>),
}

impl Provider {
    pub fn is_scoped(&self) -> bool {
        matches!(self, Provider::Scoped(_))
    }

    pub fn is_singleton(&self) -> bool {
        matches!(self, Provider::Singleton(_))
    }

    pub fn kind(&self) -> ProviderKind {
        match self {
            Provider::Scoped(_) => ProviderKind::Scoped,
            Provider::Singleton(_) => ProviderKind::Singleton
        }
    }
}