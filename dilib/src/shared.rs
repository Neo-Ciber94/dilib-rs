use crate::late_init::LateInit;
use crate::Container;
use std::any::Any;
use std::sync::Arc;

type BoxedOnceFn = Box<dyn FnOnce(&Container) -> Arc<dyn Any + Send + Sync> + Send + Sync>;

/// Provides a singleton value.
#[derive(Clone)]
pub enum Shared<'a> {
    /// A single instance of the value.
    Instance(Arc<dyn Any + Send + Sync>),
    /// A factory function to create the single value.
    Lazy(Arc<LateInit<Arc<dyn Any + Send + Sync>, &'a Container<'a>, BoxedOnceFn>>),
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
    pub fn new_factory<T, F>(f: F) -> Self
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
                    lazy.get_or_init(&container).clone().downcast().ok()
                }
            }
        }
    }
}

pub mod late_init {
    use once_cell::sync::OnceCell;
    use std::cell::Cell;
    use std::marker::PhantomData;

    pub struct LateInit<T, Arg, F = fn(Arg) -> T> {
        cell: OnceCell<T>,
        init: Cell<Option<F>>,
        _marker: PhantomData<Arg>,
    }

    unsafe impl<T: Sync, Arg: Sync, F: Send> Sync for LateInit<T, Arg, F> {}

    impl<T, Arg, F> LateInit<T, Arg, F> {
        pub const fn new(init: F) -> Self {
            LateInit {
                cell: OnceCell::new(),
                init: Cell::new(Some(init)),
                _marker: PhantomData,
            }
        }

        #[inline]
        pub fn get(&self) -> Option<&T> {
            self.cell.get()
        }

        #[inline]
        pub fn is_init(&self) -> bool {
            self.get().is_some()
        }
    }

    impl<T, Arg, F> LateInit<T, Arg, F>
    where
        F: FnOnce(Arg) -> T,
    {
        pub fn get_or_init(&self, arg: Arg) -> &T {
            self.cell.get_or_init(|| match self.init.take() {
                Some(init) => init(arg),
                None => panic!("LateInit instance has previously been poisoned"),
            })
        }
    }
}
