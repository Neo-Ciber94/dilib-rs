use crate::scoped::Scoped;
use crate::{Container, Injectable, Singleton};
use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

/// Represents the type of the provider.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ProviderKind {
    /// Is a scoped provider.
    Scoped,
    /// Is a singleton provider.
    Singleton,
}

/// Provides a [`Container`] value.
#[derive(Clone)]
pub enum Provider {
    /// A provider that returns a new value each time is requested.
    Scoped(Scoped),
    /// A provider that returns the same value each time is required.
    Singleton(Arc<dyn Any + Send + Sync>),
}

impl Provider {
    /// Returns `true` if the provider is scoped.
    pub fn is_scoped(&self) -> bool {
        matches!(self, Provider::Scoped(_))
    }

    /// Returns `true` if the provider is a singleton.
    pub fn is_singleton(&self) -> bool {
        matches!(self, Provider::Singleton(_))
    }

    /// Returns the type of this provider.
    pub fn kind(&self) -> ProviderKind {
        match self {
            Provider::Scoped(_) => ProviderKind::Scoped,
            Provider::Singleton(_) => ProviderKind::Singleton,
        }
    }

    /// Gets a value of the specified type `T` or `None` if `T` is `Injectable`.
    #[inline]
    pub fn get_scoped<T>(&self) -> Option<T>
    where
        T: Send + Sync + 'static,
    {
        match self {
            Provider::Scoped(f) if f.is_factory() => f.call_factory::<T>(),
            _ => None,
        }
    }

    /// Gets a value of the specified type `T` or `None` if `T` is not `Injectable`.
    #[inline]
    pub fn get_injectable<T>(&self, container: &Container) -> Option<T>
    where
        T: Injectable + 'static,
    {
        match self {
            Provider::Scoped(f) if f.is_injectable() => f.call_injectable::<T>(container),
            _ => None,
        }
    }

    /// Gets the singleton value of the specified type or `None`
    /// or `None` if the provider is not a singleton.
    #[inline]
    pub fn get_singleton<T>(&self) -> Option<Singleton<T>>
    where
        T: Send + Sync + 'static,
    {
        match self {
            Provider::Singleton(value) => value.clone().downcast().ok(),
            _ => None,
        }
    }
}

impl Debug for Provider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::Scoped(scoped) => write!(f, "Provider::Scoped({:?})", scoped),
            Provider::Singleton(_) => write!(f, "Provider::Singleton(..))"),
        }
    }
}
