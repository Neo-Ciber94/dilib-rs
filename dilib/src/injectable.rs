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

/// A trait for attempt to construct a type getting the dependencies from a `Container`.
pub trait TryInject : Sized {
    /// Attempts to constructs this type using the `Container`.
    fn try_inject(container: &Container) -> Result<Self, &'static str>;
}