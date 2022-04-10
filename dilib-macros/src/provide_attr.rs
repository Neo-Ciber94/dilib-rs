use crate::resolve_fn_arg::ResolvedFnArg;
use crate::scope::Scope;
use crate::target::Target;
use mattro::{MacroAttribute, Value};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use std::str::FromStr;
use syn::{AttrStyle, AttributeArgs, ItemFn, ItemStruct, ReturnType};

#[derive(Debug)]
pub struct ProvideAttribute {
    name: Option<String>,
    scope: Scope,
    target: Target,
}

impl ProvideAttribute {
    pub fn new(attr: AttributeArgs, target: Target) -> Self {
        let attr = MacroAttribute::from_attribute_args("provide", attr, AttrStyle::Outer);

        let name_value_attr = attr
            .into_name_values()
            .expect("#[provide] have invalid arguments");

        let mut map = name_value_attr
            .into_iter()
            .collect::<HashMap<String, Value>>();

        let name = map.remove_entry("name").map(|(_, value)| {
            value
                .to_string_literal()
                .expect("#[provide] 'name' must be a string literal")
        });

        let scope = map
            .remove_entry("scope")
            .map(|(_, value)| {
                value
                    .to_string_literal()
                    .expect("#[provide] 'scope' must be a string literal")
            })
            .map(|s| Scope::from_str(&s).unwrap())
            .unwrap_or(Scope::Scoped);

        // Handle unknowns key-value
        if let Some((invalid_key, _)) = map.iter().next() {
            panic!("#[provide] has invalid key: {}", invalid_key);
        }

        ProvideAttribute {
            name,
            scope,
            target,
        }
    }

    pub fn expand(self) -> proc_macro2::TokenStream {
        let name = self.name;
        let scope = self.scope;
        let target = self.target;
        let ty = target.target_type();

        let key = get_injection_key(&ty, name.as_deref());

        // We need a return type for the function
        if let Target::Fn(item_fn) = &target {
            if item_fn.sig.output == ReturnType::Default {
                panic!("function {} must have a return type", item_fn.sig.ident);
            }
        }

        let provider = match &target {
            Target::Fn(item_fn) => match scope {
                Scope::Scoped => {
                    if item_fn.sig.inputs.is_empty() {
                        get_scoped_provider(&item_fn)
                    } else {
                        get_resolved_scoped_provider(&item_fn, &ty)
                    }
                }
                Scope::Singleton => {
                    if item_fn.sig.inputs.is_empty() {
                        get_singleton_provider(&item_fn)
                    } else {
                        get_resolved_singleton_provider(&item_fn, &ty)
                    }
                }
            },
            Target::Struct(item_struct) => get_injectable_provider(&item_struct),
        };

        let add_provider = quote! {
            let mut lock = dilib::global::PROVIDERS.lock().expect("unable to get providers lock");
            let providers = lock.as_mut().expect("unable to get providers");

            providers.push(dilib::global::InjectProvider {
                key: #key,
                provider: #provider,
            });
        };

        let ctor_name = generate_fn_name(&ty, &target);

        quote! {
            #[cold]
            #[doc(hidden)]
            #[ctor::ctor]
            fn #ctor_name() {
                #add_provider
            }

            #[allow(dead_code)]
            #target
        }
    }
}

fn get_injection_key(ty: &Box<syn::Type>, name: Option<&str>) -> TokenStream {
    match name {
        Some(s) => {
            quote! {
                dilib::InjectionKey::with_name::<#ty>(#s)
            }
        }
        None => {
            quote! {
                dilib::InjectionKey::of::<#ty>()
            }
        }
    }
}

fn get_scoped_provider(item_fn: &ItemFn) -> TokenStream {
    let fn_name = item_fn.sig.ident.clone();
    quote! {
        dilib::Provider::Scoped(
            dilib::Scoped::from_factory(#fn_name)
        )
    }
}

fn get_resolved_scoped_provider(item_fn: &ItemFn, ty: &Box<syn::Type>) -> TokenStream {
    let fn_name = item_fn.sig.ident.clone();
    let resolved_args = ResolvedFnArg::from_fn(&item_fn);
    let arg_names = resolved_args
        .iter()
        .map(|arg| syn::Ident::new(&arg.arg_name, Span::call_site()))
        .collect::<Vec<_>>();

    quote! {
        dilib::Provider::Scoped(
            dilib::Scoped::from_injectable(|container: &dilib::Container| -> #ty {
                #(#resolved_args)*
                #fn_name(#(#arg_names),*)
            })
        )
    }
}

fn get_singleton_provider(item_fn: &ItemFn) -> TokenStream {
    let fn_name = item_fn.sig.ident.clone();
    let fn_call = quote! { #fn_name () };

    quote! {
        dilib::Provider::Singleton(
            dilib::Shared::new(#fn_call)
        )
    }
}

fn get_resolved_singleton_provider(item_fn: &ItemFn, ty: &Box<syn::Type>) -> TokenStream {
    let fn_name = item_fn.sig.ident.clone();
    let resolved_args = ResolvedFnArg::from_fn(&item_fn);
    let arg_names = resolved_args
        .iter()
        .map(|arg| syn::Ident::new(&arg.arg_name, Span::call_site()))
        .collect::<Vec<_>>();

    quote! {
        dilib::Provider::Singleton(
            dilib::Shared::from_factory(|container: &dilib::Container| -> #ty {
                #(#resolved_args)*
                #fn_name(#(#arg_names),*)
            })
        )
    }
}

fn get_injectable_provider(item_struct: &ItemStruct) -> TokenStream {
    let struct_name = item_struct.ident.clone();

    quote! {
        dilib::Provider::Scoped(
            dilib::Scoped::from_injectable(|container: &dilib::Container| -> #struct_name {
                    <#struct_name as dilib::Injectable> :: resolve(container)
                }
            )
        )
    }
}

fn generate_fn_name(ty: &Box<syn::Type>, target: &Target) -> syn::Ident {
    let name = match target {
        Target::Fn(item_fn) => item_fn.sig.ident.clone(),
        Target::Struct(item_struct) => item_struct.ident.clone(),
    };

    let type_name = ty
        .to_token_stream()
        .into_iter()
        .flat_map(|t| t.to_string().chars().collect::<Vec<char>>())
        .filter(|c| c.is_whitespace())
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => c,
            _ => '_',
        })
        .collect::<String>();

    let uuid = uuid::Uuid::new_v4().to_simple().to_string();
    let name = format!("{}_{}_{}", name, type_name, uuid);
    syn::Ident::new(&name, Span::call_site())
}
