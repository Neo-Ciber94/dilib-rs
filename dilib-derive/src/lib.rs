mod constructor;
mod dependency;
mod helpers;
mod keys;
mod target;
mod utils;

use crate::target::parse_derive_inject;
use proc_macro::TokenStream;

/// Provides an implementation of the `Inject` trait for the given type.
///
/// # Configuration
/// You can use `#[inject]` on the fields to configure how the values are injected.
///
/// - `#[inject(name="name")]`: To get a named dependency.
/// - `#[inject(default)]`: To set `Default::default()` for the field.
/// - `#[inject(default=literal)]`: To set the literal value for the field.
/// - `#[inject(constructor=name(arg1, arg2, ...))]`: To set the constructor to use.
/// - `#[inject(scope="scoped")]`: To get a scoped dependency. This is the default.
/// - `#[inject(scope="singleton")]`: To get a singleton dependency. If the field type is `Arc<T>` or `Singleton<T>` this is the default.
///
/// # Example
/// ```rust
/// use std::sync::{Arc, Mutex};
/// use dilib_derive::Inject;
///
/// #[derive(Inject)]
/// #[inject(constructor="new(counter, api_key, id)")]
/// struct MyService {
///     #[inject(default=1_usize)]
///     id: usize,
///
///     #[inject(name="API_KEY")]
///     api_key: String,
///
///     #[inject(scope="singleton")]
///     counter: Arc<Mutex<usize>>
/// }
///
/// impl MyService {
///     pub fn new(counter: Arc<Mutex<usize>>, api_key: String, id: usize) -> Self {
///         MyService { counter, api_key, id }
///     }
/// }
/// ```
///
#[proc_macro_derive(Inject, attributes(inject))]
pub fn derive_injectable_attribute(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item);
    parse_derive_inject(input).expand().into()
}
