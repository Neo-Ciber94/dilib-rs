use std::any::Any;
use std::sync::Arc;
use crate::scoped::Scoped;

/// Represents the type of the provider.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ProviderKind {
    /// Is a scoped provider.
    Scoped,
    /// Is a singleton provider.
    Singleton
}

/// Wraps an value or factory type.
#[derive(Clone)]
pub enum Provider {
    /// A function that produces a new value each time.
    Scoped(Scoped),
    /// A single value shared between various objects.
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