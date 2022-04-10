use crate::Container;
use std::any::Any;
use std::cell::RefCell;
use std::sync::Arc;

/// Provides a singleton value.
pub enum Shared {
    /// A single instance of the value.
    Instance(Arc<dyn Any + Send + Sync>),
    /// A factory function to create the single value.
    Lazy(RefCell<Option<Arc<dyn Fn(&Container) -> Arc<dyn Any + Send + Sync> + Send + Sync>>>),
}

impl Clone for Shared {
    fn clone(&self) -> Self {
        match self {
            Shared::Instance(instance) => Shared::Instance(Arc::clone(instance)),
            Shared::Lazy(factory) => Shared::Lazy(RefCell::clone(factory)),
        }
    }
}

impl Shared {
    /// Provides a new instance of the singleton.
    pub fn new<T>(value: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        Shared::Instance(Arc::new(value))
    }

    /// Provides a new instance of the singleton from a factory.
    pub fn new_factory<T, F>(f: F) -> Self
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        let factory = Arc::new(move |container: &Container| -> Arc<dyn Any + Send + Sync> {
            Arc::new(f(container))
        });
        Shared::Lazy(RefCell::new(Some(factory)))
    }

    pub fn is_factory(&self) -> bool {
        matches!(self, Shared::Lazy(_))
    }

    pub(crate) fn get<T>(&self) -> Option<Arc<T>>
    where
        T: Send + Sync + 'static,
    {
        match self {
            Shared::Instance(x) => x.clone().downcast().ok(),
            Shared::Lazy(_) => None,
        }
    }

    pub(crate) fn get_with<T>(&self, container: &Container) -> Option<Arc<T>>
    where
        T: Send + Sync + 'static,
    {
        match self {
            Shared::Instance(_) => None,
            Shared::Lazy(factory) => {
                let f = &*factory.take()?;
                let value = (f)(container);
                value.downcast().ok()
            }
        }
    }
}
