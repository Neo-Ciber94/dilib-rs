mod constructor;
mod dependency;
mod helpers;
mod strings;
mod target;
mod utils;

use crate::target::parse_derive_injectable;
use proc_macro::TokenStream;

#[proc_macro_derive(Injectable, attributes(inject))]
pub fn derive_injectable_attribute(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item);

    parse_derive_injectable(input)
        .emit()
        .into()
}
