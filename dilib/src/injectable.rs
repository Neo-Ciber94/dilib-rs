use crate::Container;

/// A trait for constructing a type getting the dependencies from a `Container`.
///
/// # Example
/// ```
/// use std::sync::Mutex;
/// use dilib::{Singleton, Inject, Container};
///
/// struct Greeter {
///    message: String,
///    total_greets: Singleton<Mutex<usize>>
/// }
///
/// impl Greeter {
///     fn greet(&self) {
///         println!("{}", self.message);
///         *self.total_greets.lock().unwrap() += 1;
///     }
/// }
///
/// impl Inject for Greeter {
///     fn inject(container: &Container) -> Self{
///         let message = container.get_scoped_with_name::<String>("es_msg").unwrap();
///         let total_greets = container.get_singleton_with_name::<Mutex<usize>>("count").unwrap();
///         Greeter { message, total_greets }
///     }
/// }
/// ```
pub trait Inject {
    /// Constructs this type using the `Container`.
    fn inject(container: &Container) -> Self;
}

impl<T: Default> Inject for T {
    #[inline]
    fn inject(_: &Container) -> Self {
        Self::default()
    }
}

/*
Is not decided yet if is necessary to have an `TryInject` trait.

When creating an instance from `Inject` there is only 2 options:
or create the instance or panic, is expected to be deterministic,
if we allow the operation to fail with an `Err` this open some questions:

- Why want an instance that may not be created?
- In which cases no creating an instance is something that should not panic?
- Why not just implement Inject to a `struct Fallible<T>(Result<T, Error>)`?

Checkout the related: `z_error.rs` and `z_typing.rs`
*/

// /// A trait for attempt to construct a type getting the dependencies from a `Container`.
// pub trait TryInject: Sized {
//     /// Attempts to constructs this type using the `Container`.
//     fn try_inject(container: &Container) -> Result<Self, crate::error::ResolveError>;
// }

// impl<T: TryInject> Inject for T {
//     #[inline]
//     fn inject(container: &Container) -> Self {
//         Self::try_inject(container).unwrap()
//     }
// }
