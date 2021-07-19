use crate::Container;

/// A trait for constructing a type getting the dependencies from a `Container`.
///
/// # Example
/// ```
/// use dilib::{Singleton, Injectable, Container};
///
/// struct Greeter {
///    message: String,
///    total_greets: Singleton<usize>
/// }
///
/// impl Greeter {
///     fn greet(&self) {
///         println!("{}", self.message);
///         *self.total_greets.lock().unwrap() += 1;
///     }
/// }
///
/// impl Injectable for Greeter {
///     fn resolve(container: &Container) -> Self{
///         let message = container.get_scoped_with_name::<String>("es_msg").unwrap();
///         let total_greets = container.get_singleton_with_name::<usize>("count").unwrap();
///         Greeter { message, total_greets }
///     }
/// }
/// ```
pub trait Injectable {
    /// Constructs this type using the `Container`.
    fn resolve(container: &Container) -> Self;
}