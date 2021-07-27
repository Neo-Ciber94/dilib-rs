use crate::Container;
use std::any::TypeId;
use std::ptr::NonNull;

/// Represents an `Scoped` provider which provide a new instance each time.
#[derive(Debug, Clone)]
pub struct Scoped {
    type_id: TypeId,
    inner: BoxClosure,
}

impl Scoped {
    /// Constructs a `Scoped` from a factory function.
    pub fn from_factory<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        let inner = BoxClosure::from_fn(f);
        let type_id = TypeId::of::<T>();
        Scoped { type_id, inner }
    }

    /// Constructs a `Scoped` from a `fn(&Container) -> T` function.
    pub fn from_injectable<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn(&Container) -> T + 'static,
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
    pub fn call_factory<T: 'static>(&self) -> Option<T> {
        if TypeId::of::<T>() != self.type_id {
            None
        } else {
            // SAFETY: Safe because check the return type is `T`
            unsafe {
                let f = self.inner.as_fn()?;
                let ret : T = f();
                Some(ret)
            }
        }
    }

    /// Calls the inner function using the given `Container` and returns the value.
    ///
    /// # Returns `None` if:
    /// - The inner function is not in the form `fn(&Container) -> T`.
    /// - The given type `T` don't match the return type of the factory.
    pub fn call_injectable<T: 'static>(&self, container: &Container) -> Option<T> {
        if TypeId::of::<T>() != self.type_id {
            None
        } else {
            // SAFETY: Safe because we check the return type is `T` and take an arg `&Container`
            unsafe {
                let f = self.inner.as_fn_arg::<&Container, T>()?;
                let ret : T = f(container);
                Some(ret)
            }
        }
    }

    /// Returns `true` if the inner function is in the form `fn() -> T`.
    #[inline]
    pub fn is_factory(&self) -> bool {
        !self.inner.takes_args()
    }

    /// Returns `true` if the inner function is in the form `fn(&Container) -> T`.
    #[inline]
    pub fn is_injectable(&self) -> bool {
        self.inner.takes_args()
    }
}

//todo: USE SAFE Rust
#[derive(Debug, Clone)]
enum BoxClosure {
    Fn(NonNull<dyn Fn()>),
    FnArg(NonNull<dyn Fn()>),
}

impl BoxClosure {
    pub fn from_fn<T, F>(f: F) -> Self
        where
            T: 'static,
            F: Fn() -> T + 'static,
    {
        let raw = Box::leak(Box::new(f));
        let ptr = raw as *mut dyn Fn() -> T as _;
        BoxClosure::Fn(NonNull::new(ptr).unwrap())
    }

    pub fn from_fn_arg<T, A, F>(f: F) -> Self
        where
            T: 'static,
            A: 'static,
            F: Fn(A) -> T + 'static,
    {
        let raw = Box::leak(Box::new(f));
        let ptr = raw as *mut dyn Fn(A) -> T as _;
        BoxClosure::FnArg(NonNull::new(ptr).unwrap())
    }

    pub unsafe fn as_fn<T>(&self) -> Option<&dyn Fn() -> T> {
        match self {
            BoxClosure::FnArg(_) => None,
            BoxClosure::Fn(f) => {
                // Super unsafe, we don't know if we match the type `T`
                let ptr = f.as_ptr() as *mut dyn Fn() as *mut dyn Fn() -> T;
                Some(&*ptr)
            }
        }
    }

    pub unsafe fn as_fn_arg<Arg, T>(&self) -> Option<&dyn Fn(Arg) -> T> {
        match self {
            BoxClosure::Fn(_) => None,
            BoxClosure::FnArg(f) => {
                // Super unsafe, we don't know if we match the type `T` or `Arg`
                let ptr = f.as_ptr() as *mut dyn Fn() as *mut dyn Fn(Arg) -> T;
                Some(&*ptr)
            }
        }
    }

    pub fn takes_args(&self) -> bool {
        matches!(self, BoxClosure::FnArg(_))
    }
}

impl Drop for BoxClosure {
    fn drop(&mut self) {
        unsafe {
            match self {
                BoxClosure::FnArg(x) |
                BoxClosure::Fn(x) => {
                    let raw = x.as_ptr();
                    let _ = Box::from_raw(raw);
                }
            }
        }
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
        container.add_scoped(|| String::from("hello"));

        let f = Scoped::from_injectable(|c| c.get_scoped::<String>().unwrap());
        assert!(f.is_injectable());

        let value = f.call_injectable::<String>(&container);
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
        let f = Scoped::from_injectable(|_| 0.5_f32);

        assert!(f.is_injectable());
        assert!(f.call_injectable::<bool>(&container).is_none());
    }
}