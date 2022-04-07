#![allow(unused_macros)]

use crate::Container;
use std::fmt::Debug;
use std::sync::atomic::{AtomicPtr, AtomicU8, Ordering};

static CONTAINER: AtomicPtr<Container<'static>> = AtomicPtr::new(0 as *mut Container<'static>);
static STATE: AtomicU8 = AtomicU8::new(UNINITIALIZED);

const UNINITIALIZED: u8 = 0;
const INITIALIZING: u8 = 1;
const INITIALIZED: u8 = 2;

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
            },
            InitContainerErrorKind::AlreadyInitialized => {
                write!(f, "The container was already initialized")
            }
        }
    }
}

/// Initializes the global [`Container`].
pub fn init_container<F>(f: F) -> Result<(), InitContainerError>
where
    F: FnOnce(&mut Container<'static>),
{
    match STATE.compare_exchange(
        UNINITIALIZED,
        INITIALIZING,
        Ordering::SeqCst,
        Ordering::SeqCst,
    ) {
        Ok(UNINITIALIZED) => {
            let mut container = Container::new();
            f(&mut container);
            let ptr = Box::into_raw(Box::new(container));
            CONTAINER.store(ptr, Ordering::SeqCst);
            STATE.store(INITIALIZED, Ordering::SeqCst);
            Ok(())
        }
        Ok(INITIALIZING) => {
            while STATE.load(Ordering::SeqCst) == INITIALIZING {
                std::hint::spin_loop();
            }

            Err(InitContainerError(InitContainerErrorKind::Initializing))
        }
        _ => Err(InitContainerError(
            InitContainerErrorKind::AlreadyInitialized,
        )),
    }
}

/// Returns a reference to the global [`Container`] or `None` if is not initialized.
pub fn get_container() -> Option<&'static Container<'static>> {
    match STATE.load(Ordering::SeqCst) {
        INITIALIZED => unsafe { Some(&*CONTAINER.load(Ordering::SeqCst)) },
        _ => None,
    }
}

/// Returns a scoped value from the global [`Container`].
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

    (trait $trait_type:ident $(<$($generic:ident),+>)?, $name:expr) => {
        let container = $crate::global::get_container().expect("The container is not initialized");
        $crate::get_scoped_trait_with_name!(container, $trait_type $(<$($generic),+>)?, $name)
    };
}

/// Returns a singleton value from the global [`Container`].
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

#[cfg(test)]
mod tests {
    use crate::global::{get_container, init_container};
    use crate::{register_scoped_trait, register_singleton_trait};
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
            container.add_scoped(|| String::from("Hello World"));
            container.add_singleton(Mutex::new(5_i32));
            register_singleton_trait!(container, Greeter, EnglishGreeter);
            register_scoped_trait!(container, Greeter, SpanishGreeter);
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

        let r3 = get_scoped!(trait Greeter).unwrap();
        assert_eq!(r3.greet(), "Hola, mundo!");

        let r4 = get_singleton!(trait Greeter).unwrap();
        assert_eq!(r4.greet(), "Hello, world!");
    }
}
