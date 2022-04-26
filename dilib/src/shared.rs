use std::any::Any;
use std::sync::Arc;

#[cfg(feature = "lazy")]
type BoxedOnceFn = Box<dyn FnOnce(&crate::Container) -> Arc<dyn Any + Send + Sync> + Send + Sync>;

#[cfg(feature = "lazy")]
use {crate::Container, late_init::LateInit};

/// Provides a singleton value.
#[derive(Clone)]
pub enum Shared<'a> {
    /// A single instance of the value.
    Instance(Arc<dyn Any + Send + Sync>),

    #[cfg(feature = "lazy")]
    /// A factory function to create the single value.
    Lazy(Arc<LateInit<Arc<dyn Any + Send + Sync>, &'a Container<'a>, BoxedOnceFn>>),

    #[doc(hidden)]
    #[cfg(not(feature = "lazy"))]
    __NonExhaustive(&'a std::marker::PhantomData<()>),
}

impl<'a> Shared<'a> {
    /// Provides a new instance of the singleton.
    pub fn new<T>(value: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        Shared::Instance(Arc::new(value))
    }

    /// Provides a new instance of the singleton from a factory.
    #[cfg(feature = "lazy")]
    pub fn new_lazy<T, F>(f: F) -> Self
    where
        T: Send + Sync + 'static,
        F: FnOnce(&Container) -> T + Send + Sync + 'static,
    {
        let factory = move |container: &Container| -> Arc<dyn Any + Send + Sync> {
            let value = f(container);
            Arc::new(value)
        };

        let boxed: BoxedOnceFn = Box::new(factory);
        let lazy = LateInit::new(boxed);
        Shared::Lazy(Arc::new(lazy))
    }

    pub(crate) fn get<T>(&self) -> Option<Arc<T>>
    where
        T: Send + Sync + 'static,
    {
        match self {
            Shared::Instance(x) => x.clone().downcast().ok(),
            #[cfg(feature = "lazy")]
            Shared::Lazy(_) => None,
            #[cfg(not(feature = "lazy"))]
            _ => None,
        }
    }

    #[cfg(feature = "lazy")]
    pub(crate) fn get_with<T>(&self, container: &'a Container<'a>) -> Option<Arc<T>>
    where
        T: Send + Sync + 'static,
    {
        match self {
            Shared::Instance(_) => None,
            Shared::Lazy(lazy) => {
                if lazy.is_init() {
                    lazy.get().and_then(|x| x.clone().downcast().ok())
                } else {
                    lazy.get_or_init(container).clone().downcast().ok()
                }
            }

            #[cfg(not(feature = "lazy"))]
            _ => None,
        }
    }
}

/// A lazy evaluated cell.
#[cfg(feature = "lazy")]
pub mod late_init {
    use once_cell::sync::OnceCell;
    use std::cell::Cell;
    use std::marker::PhantomData;

    /// A `Lazy<T>` that takes an argument.
    pub struct LateInit<T, Arg, F = fn(Arg) -> T> {
        cell: OnceCell<T>,
        init: Cell<Option<F>>,
        _marker: PhantomData<Arg>,
    }

    unsafe impl<T: Sync, Arg: Sync, F: Send> Sync for LateInit<T, Arg, F> {}

    impl<T, Arg, F> LateInit<T, Arg, F> {
        /// Constructs a new `LateInit<T>` with the given function.
        pub const fn new(init: F) -> Self {
            LateInit {
                cell: OnceCell::new(),
                init: Cell::new(Some(init)),
                _marker: PhantomData,
            }
        }

        /// Returns `Some(&T)` if the value has been initialized, `None` otherwise.
        #[inline]
        pub fn get(&self) -> Option<&T> {
            self.cell.get()
        }

        /// Returns `true` if the value has been initialized.
        #[inline]
        pub fn is_init(&self) -> bool {
            self.get().is_some()
        }
    }

    impl<T, Arg, F> LateInit<T, Arg, F>
    where
        F: FnOnce(Arg) -> T,
    {
        /// Initializes the instance if it has not been initialized and returns a reference to the value.
        pub fn get_or_init(&self, arg: Arg) -> &T {
            self.cell.get_or_init(|| match self.init.take() {
                Some(init) => init(arg),
                None => panic!("LateInit instance has previously been poisoned"),
            })
        }
    }
}
