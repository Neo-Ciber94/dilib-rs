mod provide_attr;
mod resolve_fn_arg;
mod scope;
mod target;

use crate::provide_attr::ProvideAttribute;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use target::Target;

#[proc_macro_attribute]
pub fn provide(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let target = parse_macro_input!(item as Target);
    ProvideAttribute::new(attr, target).expand().into()
}

#[proc_macro_attribute]
pub fn inject(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/*
#[provide] should generate a #[ctor] macro that allow to register
a Provider to the global Container. This require the `global` feature enable, which
will be enable with the unstable_provide feature.

#[provide]
fn hello() -> String {
    "Hello".to_string()
}

#[provide]
#[derive(Injectable)]
struct MyClass {
    name: String,
}

#[provide] only can bt used in structs and will assume the struct is Injectable
and in functions.

Options:
- name: Name for the provider
- scope: Scope for the provider "scoped" or "singleton", default is "scoped"
- order: Order to run the provider, default is 0. Providers will run after init_container()

If provide is used in a function like:

#[provide]
fn compute(hello: String, mul: Singleton<u32>) -> usize {
   todo!()
}

The code generation will try to get all the references in the parameters:

#[ctor]
fn compute_7ddbb6309d14a74d92e31b26f3ff5454dfa0708b() {
    PROVIDERS.push(Provider::Scope(
        Scope::from_injectable(|c| {
            let hello = c.get::<String>("hello").unwrap();
            let mul = c.get::<u32>("mul").unwrap();
            compute(hello, mul)
        })
    ))
}

#[provide]
fn echo(#[provide_from("name")] hello: String) -> String {
}

*/
