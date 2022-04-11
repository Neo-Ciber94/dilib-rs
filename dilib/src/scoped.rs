use crate::error::ResolveError;
use crate::Container;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

type TryInjectResult = Result<Box<dyn Any + Send + Sync>, ResolveError>;

/// Represents an `Scoped` provider which provide a new instance each time.
#[derive(Clone)]
pub enum Scoped {
    Factory(Arc<dyn Fn() -> Box<dyn Any + Send + Sync> + Send + Sync>),
    Inject(Arc<dyn Fn(&Container) -> Box<dyn Any + Send + Sync> + Send + Sync>),
    TryInject(Arc<dyn Fn(&Container) -> TryInjectResult + Send + Sync>),
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
    pub fn from_inject<T, F>(f: F) -> Self
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        let factory =
            Arc::new(move |c: &Container| -> Box<dyn Any + Send + Sync> { Box::new(f(c)) });

        Scoped::Inject(factory)
    }

    /// Creates a new `Scoped` provider from a try-inject function.
    pub fn from_try_inject<T, F>(f: F) -> Self
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> Result<T, ResolveError> + Send + Sync + 'static,
    {
        let factory = Arc::new(
            move |c: &Container| -> Result<Box<dyn Any + Send + Sync>, ResolveError> {
                let x = f(c)?;
                Ok(Box::new(x))
            },
        );

        Scoped::TryInject(factory)
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

    pub(crate) fn call_inject<T>(&self, container: &Container) -> Option<T>
    where
        T: 'static,
    {
        match self {
            Scoped::Inject(f) => {
                let x: Box<dyn Any + Send + Sync> = f.as_ref()(container);
                x.downcast().map(|x| *x).ok()
            }
            _ => None,
        }
    }

    pub(crate) fn call_try_inject<T>(&self, container: &Container) -> Result<T, ResolveError>
    where
        T: 'static,
    {
        match self {
            Scoped::TryInject(f) => {
                let x: Box<dyn Any + Send + Sync> = f.as_ref()(container)?;
                x.downcast().map(|x| *x).map_err(|_| {
                    let desc = format!("expected type: {}", std::any::type_name::<T>());
                    ResolveError::with_description(
                        "called call_try_inject on an invalid type",
                        desc,
                    )
                })
            }
            _ => Err(ResolveError::with_message(
                "type is not a Scoped::TryInject",
            )),
        }
    }
}

impl Debug for Scoped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scoped::Factory(_) => write!(f, "Scoped::Factory(..)"),
            Scoped::Inject(_) => write!(f, "Scoped::Inject(..)"),
            Scoped::TryInject(_) => write!(f, "Scoped::TryInject(..)"),
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
    fn call_inject_test() {
        let mut container = Container::new();
        container.add_singleton(5_i32).unwrap();

        let inject = Scoped::from_inject(|c| c.get::<i32>().unwrap().cloned() + 1);
        assert_eq!(inject.call_inject(&container), Some(6));
    }

    #[test]
    fn call_try_inject_test() {
        let mut container = Container::new();
        container.add_singleton(5_i32).unwrap();

        let inject = Scoped::from_try_inject(|c| Ok(c.get::<i32>().unwrap().cloned() + 1));
        assert_eq!(inject.call_try_inject(&container).ok(), Some(6));
    }
}
