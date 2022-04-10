mod provide_attr;
mod resolve_fn_arg;
mod scope;
mod target;

use crate::provide_attr::ProvideAttribute;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use target::Target;

/// Adds a provider from a `function` or `struct` to the global `Container`.
///
/// # Arguments
/// - `name`: The name of the provider.
/// - `scope`: The scope 'singleton' or 'scoped', defaults to 'scoped'.
///
/// # Example
///
/// ```rust
/// use dilib::{macros::provide, Injectable, Singleton, global::init_container, get_singleton, get_scoped};
/// use std::sync::Mutex;
///
/// #[provide]
/// fn message() -> String {
///    "Current count".to_string()
/// }
///
/// #[provide(scope = "singleton", name="count")]
/// fn counter() -> Mutex<usize> {
///   Mutex::new(0)
/// }
///
/// #[derive(Injectable)]
/// #[provide] // provide should resolve after the derive
/// struct PrintCount {
///     #[inject(name="count")]
///     counter: Singleton<Mutex<usize>>,
///     msg: String
/// }
///
/// impl PrintCount {
///     fn print(&self) -> String {
///         let count = self.counter.lock().unwrap();
///         format!("{} {}", self.msg, count)
///     }
/// }
///
/// // Required to register the providers
/// init_container(|_|{}).unwrap();
///
/// let counter = get_singleton!(Mutex<usize>, "count").expect("Could not get counter");
/// *counter.lock().unwrap() += 5;
///
/// let print_count = get_scoped!(PrintCount).expect("Could not get print count");
/// assert_eq!(print_count.print(), "Current count 5");
/// ```
#[proc_macro_attribute]
pub fn provide(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let target = parse_macro_input!(item as Target);
    ProvideAttribute::new(attr, target).expand().into()
}

/// Specify the dependency to inject to a function argument.
///
/// # Example
/// ```rust,no_run
/// use std::sync::Arc;
/// use dilib::macros::{provide, inject};
///
/// #[provide]
/// #[inject(value, name="initial_value")]
/// fn compute(hash: String, value: Arc<usize>) -> usize {
///    0 // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn inject(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}
