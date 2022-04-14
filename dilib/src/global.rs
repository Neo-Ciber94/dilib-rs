use crate::Container;
use std::fmt::Debug;
use std::sync::atomic::{AtomicPtr, AtomicU8, Ordering};

#[cfg(feature = "unstable_provide")]
use {
    crate::{InjectionKey, Provider},
    once_cell::sync::Lazy,
    std::sync::Mutex,
};

static CONTAINER: GlobalContainer = GlobalContainer::new();

const UNINITIALIZED: u8 = 0;
const INITIALIZING: u8 = 1;
const INITIALIZED: u8 = 2;

struct GlobalContainer {
    container: AtomicPtr<Container<'static>>,
    state: AtomicU8,
}

impl GlobalContainer {
    const fn new() -> Self {
        Self {
            container: AtomicPtr::new(std::ptr::null_mut()),
            state: AtomicU8::new(UNINITIALIZED),
        }
    }

    fn initialize<F>(&self, init: F) -> Result<(), InitContainerError>
    where
        F: FnOnce(&mut Container<'static>),
    {
        let state = &self.state;

        match state.compare_exchange(
            UNINITIALIZED,
            INITIALIZING,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ) {
            Ok(UNINITIALIZED) => {
                let mut container = Container::new();
                init(&mut container);
                let ptr = Box::into_raw(Box::new(container));

                self.container.store(ptr, Ordering::SeqCst);
                state.store(INITIALIZED, Ordering::SeqCst);
                Ok(())
            }
            Ok(INITIALIZING) => {
                while state.load(Ordering::SeqCst) == INITIALIZING {
                    std::hint::spin_loop();
                }

                Err(InitContainerError(InitContainerErrorKind::Initializing))
            }
            _ => Err(InitContainerError(
                InitContainerErrorKind::AlreadyInitialized,
            )),
        }
    }

    fn get(&self) -> Option<&'static Container<'static>> {
        match self.state.load(Ordering::SeqCst) {
            INITIALIZED => unsafe { Some(&*self.container.load(Ordering::SeqCst)) },
            _ => None,
        }
    }

    // This method should never be exposed, global container should be read-only after initialization
    #[allow(dead_code)]
    fn get_mut(&self) -> Option<&'static mut Container<'static>> {
        match self.state.load(Ordering::SeqCst) {
            INITIALIZED => unsafe { Some(&mut *self.container.load(Ordering::SeqCst)) },
            _ => None,
        }
    }
}

/// Container initialization errors.
pub enum InitContainerErrorKind {
    /// The container was initializing.
    Initializing,
    /// The container was already initialized.
    AlreadyInitialized,
}

/// Error returned when the container initialization failed.
pub struct InitContainerError(InitContainerErrorKind);

impl Debug for InitContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            InitContainerErrorKind::Initializing => {
                write!(f, "The container was initializing")
            }
            InitContainerErrorKind::AlreadyInitialized => {
                write!(f, "The container was already initialized")
            }
        }
    }
}

// A provider to be injected by #[provide]
#[doc(hidden)]
#[cfg(feature = "unstable_provide")]
pub struct InjectProvider {
    // The actual provider
    pub provider: Provider<'static>,
    // The key used to inject the provider
    pub key: InjectionKey<'static>,
}

// List of providers to be added to the global container
#[doc(hidden)]
#[cfg(feature = "unstable_provide")]
pub static PROVIDERS: Lazy<Mutex<Option<Vec<InjectProvider>>>> =
    Lazy::new(|| Mutex::new(Some(vec![])));

/// Initializes the global [`Container`].
#[cold]
pub fn init_container<F>(f: F) -> Result<(), InitContainerError>
where
    F: FnOnce(&mut Container<'static>),
{
    match CONTAINER.initialize(f) {
        Ok(_) => {
            #[cfg(feature = "unstable_provide")]
            {
                let mut lock = PROVIDERS.lock().unwrap();
                let providers = lock.take().unwrap();
                let container = CONTAINER.get_mut().unwrap();

                for InjectProvider { key, provider, .. } in providers {
                    container
                        .add_provider_internal(key.clone(), provider)
                        .unwrap_or_else(|_| panic!("Failed to add provider for key: {:?}", key));
                }
            }

            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// Returns a reference to the global [`Container`] or `None` if is not initialized.
#[inline]
pub fn get_container() -> Option<&'static Container<'static>> {
    CONTAINER.get()
}

/// Returns a scoped value from the global [`Container`] or `None` if is not in the container.
#[macro_export]
macro_rules! get_scoped {
    ($scoped_type:ty) => {
        $crate::global::get_container()
            .expect("The container is not initialized")
            .get_scoped::<$scoped_type>()
    };

    ($scoped_type:ty, $name:expr) => {
        $crate::global::get_container()
            .expect("The container is not initialized")
            .get_scoped_with_name::<$scoped_type>($name)
    };

   (trait $trait_type:ident $(<$($generic:ident),+>)?) => {{
        let container = $crate::global::get_container().expect("The container is not initialized");
        $crate::get_scoped_trait!(container, $trait_type $(<$($generic),+>)?)
   }};

   (trait $trait_type:ident $(<$($generic:ident),+>)?, $name:expr) => {{
        let container = $crate::global::get_container().expect("The container is not initialized");
        $crate::get_scoped_trait!(container, $trait_type $(<$($generic),+>)?, $name)
   }};
}

/// Returns a singleton value from the global [`Container`] or `None` if is not in the container.
#[macro_export]
macro_rules! get_singleton {
    ($singleton_type:ty) => {
        $crate::global::get_container()
            .expect("The container is not initialized")
            .get_singleton::<$singleton_type>()
    };

    ($singleton_type:ty, $name:expr) => {
        $crate::global::get_container()
            .expect("The container is not initialized")
            .get_singleton_with_name::<$singleton_type>($name)
    };

    (trait $trait_type:ident $(<$($generic:ident),+>)?) => {{
        let container = $crate::global::get_container().expect("The container is not initialized");
        $crate::get_singleton_trait!(container, $trait_type $(<$($generic),+>)?)
    }};

    (trait $trait_type:ident $(<$($generic:ident),+>)?, $name:expr) => {{
        let container = $crate::global::get_container().expect("The container is not initialized");
        $crate::get_singleton_trait!(container, $trait_type $(<$($generic),+>)?, $name)
    }};
}

/// Returns a value from the [`Container`] or `None` if is not in the container.
#[macro_export]
macro_rules! resolve {
    ($type:ty) => {
        $crate::global::get_container()
            .expect("The container is not initialized")
            .get::<$type>()
    };

    ($type:ty, $name:expr) => {
        $crate::global::get_container()
            .expect("The container is not initialized")
            .get_with_name::<$type>($name)
    };

    (trait $trait_type:ident $(<$($generic:ident),+>)?) => {{
        let container = $crate::global::get_container().expect("The container is not initialized");
        $crate::get_resolved_trait!(container, $trait_type $(<$($generic),+>)?)
    }};

    (trait $trait_type:ident $(<$($generic:ident),+>)?, $name:expr) => {{
        let container = $crate::global::get_container().expect("The container is not initialized");
        $crate::get_resolved_trait!(container, $trait_type $(<$($generic),+>)?, $name)
    }};
}

#[cfg(test)]
mod tests {
    use crate::global::{get_container, init_container, GlobalContainer};
    use crate::{add_scoped_trait, add_singleton_trait};
    use std::sync::Mutex;

    pub trait Greeter {
        fn greet(&self) -> &str;
    }

    struct EnglishGreeter;
    impl Greeter for EnglishGreeter {
        fn greet(&self) -> &str {
            "Hello, world!"
        }
    }

    struct SpanishGreeter;
    impl Greeter for SpanishGreeter {
        fn greet(&self) -> &str {
            "Hola, mundo!"
        }
    }

    #[test]
    fn global_container_test() {
        init_container(|container| {
            container
                .add_scoped(|| String::from("Hello World"))
                .unwrap();
            container.add_singleton(Mutex::new(5_i32)).unwrap();
            add_singleton_trait!(container, Greeter => EnglishGreeter).unwrap();
            add_scoped_trait!(container, "es", Greeter => SpanishGreeter).unwrap();
        })
        .unwrap();

        let container = get_container().unwrap();

        assert_eq!(
            container.get_scoped::<String>().unwrap(),
            "Hello World".to_owned()
        );
        assert_eq!(
            *container
                .get_singleton::<Mutex<i32>>()
                .unwrap()
                .lock()
                .unwrap(),
            5_i32
        );

        let r1 = get_scoped!(String).unwrap();
        assert_eq!(r1, "Hello World".to_owned());

        let r2 = get_singleton!(Mutex<i32>).unwrap();
        assert_eq!(*r2.lock().unwrap(), 5_i32);

        let r3 = get_scoped!(trait Greeter, "es").unwrap();
        assert_eq!(r3.greet(), "Hola, mundo!");

        let r4 = get_singleton!(trait Greeter).unwrap();
        assert_eq!(r4.greet(), "Hello, world!");

        let r5 = resolve!(String).unwrap();
        let r6 = resolve!(Mutex<i32>).unwrap();

        assert_eq!(&*r5, "Hello World");
        assert_eq!(*r6.lock().unwrap(), 5_i32);
    }

    #[test]
    fn no_initialized_test() {
        static GLOBAL_CONTAINER: GlobalContainer = GlobalContainer::new();
        assert!(GLOBAL_CONTAINER.get().is_none());

        GLOBAL_CONTAINER.initialize(|_| {}).unwrap();
        assert!(GLOBAL_CONTAINER.get().is_some());
    }
}
