mod target;
mod dependency;
mod constructor;
mod helpers;
mod utils;
mod strings;

use proc_macro::TokenStream;
use crate::target::parse_derive_injectable;

#[proc_macro_derive(Injectable, attributes(inject))]
pub fn derive_injectable_attribute(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item);

    parse_derive_injectable(input)
        .emit()
        .into()
}