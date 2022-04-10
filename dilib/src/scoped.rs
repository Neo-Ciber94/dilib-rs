use crate::Container;
use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

/// Represents an `Scoped` provider which provide a new instance each time.
#[derive(Debug, Clone)]
pub struct Scoped {
    type_id: TypeId, // This TypeId may be redundant
    inner: BoxClosure,
}

impl Scoped {
    /// Constructs a `Scoped` from a factory function.
    pub fn from_factory<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn() -> T + Send + Sync + 'static,
    {
        let inner = BoxClosure::from_fn(f);
        let type_id = TypeId::of::<T>();
        Scoped { type_id, inner }
    }

    /// Constructs a `Scoped` from a `fn(&Container) -> T` function.
    pub fn from_inject<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        let inner = BoxClosure::from_fn_arg(f);
        let type_id = TypeId::of::<T>();
        Scoped { type_id, inner }
    }

    /// Calls the inner function and returns the value.
    ///
    /// # Returns `None` if:
    /// - The inner function is not in the form `fn() -> T`.
    /// - The given type `T` don't match the return type of the factory.
    pub fn call_factory<T>(&self) -> Option<T>
    where
        T: Send + Sync + 'static,
    {
        if TypeId::of::<T>() != self.type_id {
            None
        } else {
            let value = self.inner.call::<T>().ok()?;
            Some(value)
        }
    }

    /// Calls the inner function using the given `Container` and returns the value.
    ///
    /// # Returns `None` if:
    /// - The inner function is not in the form `fn(&Container) -> T`.
    /// - The given type `T` don't match the return type of the factory.
    pub fn call_inject<T: 'static>(&self, container: &Container) -> Option<T> {
        if TypeId::of::<T>() != self.type_id {
            None
        } else {
            let value = self.inner.call_with::<T>(container).ok()?;
            Some(value)
        }
    }

    /// Returns `true` if the inner function is in the form `fn() -> T`.
    #[inline]
    pub fn is_factory(&self) -> bool {
        !self.inner.takes_args()
    }

    /// Returns `true` if the inner function is in the form `fn(&Container) -> T`.
    #[inline]
    pub fn is_inject(&self) -> bool {
        self.inner.takes_args()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct InvalidFunctionType;

#[derive(Clone)]
enum BoxClosure {
    Fn(Arc<dyn Fn() -> Box<dyn Any> + Send + Sync>),
    FnArg(Arc<dyn Fn(&Container) -> Box<dyn Any> + Send + Sync>),
}

impl Debug for BoxClosure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxClosure::Fn(_) => write!(f, "BoxClosure::Fn(..)"),
            BoxClosure::FnArg(_) => write!(f, "BoxClosure::FnArg(..)"),
        }
    }
}

impl BoxClosure {
    pub fn from_fn<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn() -> T + Send + Sync + 'static,
    {
        let func: Arc<dyn Fn() -> Box<dyn Any> + Send + Sync> = Arc::new(move || Box::new(f()));
        BoxClosure::Fn(func)
    }

    pub fn from_fn_arg<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        let func: Arc<dyn Fn(&Container) -> Box<dyn Any> + Send + Sync> =
            Arc::new(move |c: &Container| Box::new(f(c)));
        BoxClosure::FnArg(func)
    }

    pub fn call<T: 'static>(&self) -> Result<T, InvalidFunctionType> {
        match self {
            BoxClosure::FnArg(_) => Err(InvalidFunctionType),
            BoxClosure::Fn(func) => {
                let value: Box<dyn Any> = func.as_ref()();
                value
                    .downcast()
                    .map(|x| *x)
                    .map_err(|_| InvalidFunctionType)
            }
        }
    }

    pub fn call_with<T: 'static>(&self, container: &Container) -> Result<T, InvalidFunctionType> {
        match self {
            BoxClosure::Fn(_) => Err(InvalidFunctionType),
            BoxClosure::FnArg(func) => {
                let value: Box<dyn Any> = func.as_ref()(container);
                value
                    .downcast()
                    .map(|x| *x)
                    .map_err(|_| InvalidFunctionType)
            }
        }
    }

    pub fn takes_args(&self) -> bool {
        matches!(self, BoxClosure::FnArg(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_factory_test() {
        let f = Scoped::from_factory(|| 123_i32);
        assert!(f.is_factory());

        let value = f.call_factory::<i32>();
        assert_eq!(value, Some(123_i32));
    }

    #[test]
    fn call_injectable_test() {
        let mut container = Container::new();
        container.add_scoped(|| String::from("hello")).unwrap();

        let f = Scoped::from_inject(|c| c.get_scoped::<String>().unwrap());
        assert!(f.is_inject());

        let value = f.call_inject::<String>(&container);
        assert_eq!(value, Some("hello".to_string()));
    }

    #[test]
    fn invalid_type_factory_test() {
        let f = Scoped::from_factory(|| true);

        assert!(f.is_factory());
        assert!(f.call_factory::<i32>().is_none());
    }

    #[test]
    fn invalid_type_injectable_test() {
        let container = Container::new();
        let f = Scoped::from_inject(|_| 0.5_f32);

        assert!(f.is_inject());
        assert!(f.call_inject::<bool>(&container).is_none());
    }
}
