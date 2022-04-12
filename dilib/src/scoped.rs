use crate::Container;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

/// Represents an `Scoped` provider which provide a new instance each time.
#[derive(Clone)]
pub enum Scoped {
    Factory(Arc<dyn Fn() -> Box<dyn Any + Send + Sync> + Send + Sync>),
    Construct(Arc<dyn Fn(&Container) -> Box<dyn Any + Send + Sync> + Send + Sync>),
}

impl Scoped {
    /// Creates a new `Scoped` provider from a factory function.
    pub fn from_factory<T, F>(f: F) -> Self
    where
        T: Send + Sync + 'static,
        F: Fn() -> T + Send + Sync + 'static,
    {
        let factory = Arc::new(move || -> Box<dyn Any + Send + Sync> { Box::new(f()) });

        Scoped::Factory(factory)
    }

    /// Creates a new `Scoped` provider from an inject function.
    pub fn from_construct<T, F>(f: F) -> Self
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        let factory =
            Arc::new(move |c: &Container| -> Box<dyn Any + Send + Sync> { Box::new(f(c)) });

        Scoped::Construct(factory)
    }

    pub(crate) fn call_factory<T>(&self) -> Option<T>
    where
        T: Send + Sync + 'static,
    {
        match self {
            Scoped::Factory(f) => {
                let x: Box<dyn Any + Send + Sync> = f.as_ref()();
                x.downcast().map(|x| *x).ok()
            }
            _ => None,
        }
    }

    pub(crate) fn call_construct<T>(&self, container: &Container) -> Option<T>
    where
        T: 'static,
    {
        match self {
            Scoped::Construct(f) => {
                let x: Box<dyn Any + Send + Sync> = f.as_ref()(container);
                x.downcast().map(|x| *x).ok()
            }
            _ => None,
        }
    }
}

impl Debug for Scoped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scoped::Factory(_) => write!(f, "Scoped::Factory(..)"),
            Scoped::Construct(_) => write!(f, "Scoped::Inject(..)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scoped::Scoped;
    use crate::Container;

    #[test]
    fn call_factory_test() {
        let factory = Scoped::from_factory(|| "hello".to_string());
        assert_eq!(factory.call_factory::<String>(), Some("hello".to_string()));
    }

    #[test]
    fn call_construct_test() {
        let mut container = Container::new();
        container.add_singleton(5_i32).unwrap();

        let inject = Scoped::from_construct(|c| c.get::<i32>().unwrap().cloned() + 1);
        assert_eq!(inject.call_construct(&container), Some(6));
    }
}
