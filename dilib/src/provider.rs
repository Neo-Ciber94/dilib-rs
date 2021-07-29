use crate::scoped::Scoped;
use crate::{Container, Injectable, Singleton};
use std::any::Any;
use std::sync::Arc;

/// Represents the type of the provider.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ProviderKind {
    /// Is a scoped provider.
    Scoped,
    /// Is a singleton provider.
    Singleton,
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
        T: 'static,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_scoped_test(){
        let mut container = Container::new();
        container.add_scoped(|| 30_i32);
        container.add_scoped(|| Some(0.5_f32));

        let v1 = container.get_scoped::<i32>().unwrap();
        let v2 = container.get_scoped::<Option<f32>>().unwrap();

        assert_eq!(v1, 30_i32);
        assert_eq!(v2, Some(0.5_f32));
    }

    #[test]
    fn get_injectable_test(){
        #[derive(Debug, Eq, PartialEq)]
        struct IntWrapper(i32);
        impl Injectable for IntWrapper {
            fn resolve(container: &Container) -> Self {
                let value = container.get_scoped::<i32>().unwrap();
                IntWrapper(value)
            }
        }

        let mut container = Container::new();
        container.add_scoped(|| 25_i32);
        container.add_deps::<IntWrapper>();

        let v1 = container.get_scoped::<IntWrapper>().unwrap();
        assert_eq!(v1, IntWrapper(25_i32));
    }

    #[test]
    fn get_singleton_test(){
        let mut container = Container::new();
        container.add_singleton(String::from("red"));

        let value = container.get_singleton::<String>().unwrap();
        assert_eq!(*value.lock().unwrap(), String::from("red"));
    }
}
