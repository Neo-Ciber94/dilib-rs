use crate::scoped::Scoped;
use crate::{Container, Inject, Shared, Singleton};
use std::fmt::{Debug, Formatter};

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
pub enum Provider<'a> {
    /// A provider that returns a new value each time is requested.
    Scoped(Scoped),
    /// A provider that returns the same value each time is required.
    Singleton(Shared<'a>),
}

impl<'a> Provider<'a> {
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

    /// Gets a value of the specified type `T` or `None` if `T` is `Inject`.
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

    /// Gets a value of the specified type `T` or `None` if `T` is not `Inject`.
    #[inline]
    pub fn get_inject<T>(&self, container: &Container) -> Option<T>
    where
        T: Inject + 'static,
    {
        match self {
            Provider::Scoped(f) if f.is_inject() => f.call_inject::<T>(container),
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
            Provider::Singleton(x) => x.get(),
            _ => None,
        }
    }

    #[inline]
    #[cfg(feature = "lazy")]
    pub fn get_singleton_with<T>(&self, container: &Container) -> Option<Singleton<T>>
    where
        T: Send + Sync + 'static,
    {
        match self {
            Provider::Singleton(x) => x.get_with(container),
            _ => None,
        }
    }
}

impl Debug for Provider<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::Scoped(scoped) => write!(f, "Provider::Scoped({:?})", scoped),
            Provider::Singleton(_) => write!(f, "Provider::Singleton(..))"),
        }
    }
}
