use crate::keys;
use crate::resolve_fn_arg::ResolvedFnArg;
use crate::scope::Scope;
use crate::target::Target;
use crate::utils::format_tokens;
use mattro::{MacroAttribute, Value};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{AttrStyle, AttributeArgs, ItemFn, ItemStruct, ReturnType, Type};

#[derive(Debug)]
pub struct ProvideAttribute {
    name: Option<String>,
    scope: Scope,
    target: Target,
    bind: Option<Box<Type>>,
}

impl ProvideAttribute {
    pub fn new(attr: AttributeArgs, target: Target) -> Self {
        let attr = MacroAttribute::from_attribute_args(keys::PROVIDE, attr, AttrStyle::Outer);

        let name_value_attr = attr
            .into_name_values()
            .unwrap_or_else(|_| panic!("#[{}] have invalid arguments", keys::PROVIDE));

        let mut map = name_value_attr
            .into_iter()
            .collect::<HashMap<String, Value>>();

        let name = map.remove_entry(keys::NAME).map(|(_, value)| {
            value.to_string_literal().unwrap_or_else(|| {
                panic!(
                    "#[{}] '{}' must be a string literal",
                    keys::PROVIDE,
                    keys::NAME
                )
            })
        });

        let scope = map
            .remove_entry("scope")
            .map(|(_, value)| {
                value.to_string_literal().unwrap_or_else(|| {
                    panic!(
                        "#[{}] '{}' must be a string literal",
                        keys::PROVIDE,
                        keys::SCOPE
                    )
                })
            })
            .map(|s| Scope::from_str(&s))
            .unwrap_or(Scope::Scoped);

        let bind = map.remove_entry(keys::BIND).map(|(_, value)| {
            let type_string = value.to_string_literal().unwrap_or_else(|| {
                panic!(
                    "#[{}] '{}' must be a string literal",
                    keys::PROVIDE,
                    keys::BIND
                )
            });

            // We need: Box<dyn TraitType + Send + Sync>
            let boxed_type = format!("std::boxed::Box<dyn {} + Send + Sync>", type_string);

            syn::parse_str::<Box<Type>>(&boxed_type)
                .unwrap_or_else(|_| panic!("'{}' is not a valid trait type", type_string))
        });

        // Handle unknowns key-value
        if let Some((invalid_key, _)) = map.iter().next() {
            panic!("#[{}] has invalid key: {}", keys::PROVIDE, invalid_key);
        }

        ProvideAttribute {
            name,
            scope,
            target,
            bind,
        }
    }

    pub fn expand(self) -> proc_macro2::TokenStream {
        let name = self.name;
        let scope = self.scope;
        let target = self.target;
        let bind = self.bind.as_deref();
        let ty = target.target_type();

        let key = get_injection_key(bind.unwrap_or(&ty), name.as_deref());

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
                        get_scoped_provider(item_fn, bind)
                    } else {
                        get_resolved_scoped_provider(item_fn, &ty, bind)
                    }
                }
                Scope::Singleton => {
                    if item_fn.sig.inputs.is_empty() {
                        get_singleton_provider(item_fn, bind)
                    } else {
                        get_resolved_singleton_provider(item_fn, &ty, bind)
                    }
                }
            },
            Target::Struct(item_struct) => match scope {
                Scope::Scoped => get_inject_provider(item_struct, bind),
                Scope::Singleton => get_singleton_inject_provider(item_struct, bind),
            },
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
            // We hide the generated function
            const _: () = {
                #[cold]
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[allow(dead_code)]
                #[ctor::ctor]
                fn #ctor_name() {
                    #add_provider
                }
            };

            // Let the rest of the code the same
            #target
        }
    }
}

fn generate_fn_name(ty: &syn::Type, target: &Target) -> syn::Ident {
    fn sanitize_type_name<T: ToTokens>(t: &T) -> String {
        let mut tokens = format_tokens(t)
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => c,
                _ => '_',
            })
            .collect::<Vec<char>>();

        if tokens.len() == 1 && tokens[0] == '_' {
            return String::new();
        }

        // Adds the separator <name>_<type>
        tokens.insert(0, '_');
        tokens.dedup();
        let s = tokens.into_iter().collect::<String>();
        s.trim_end_matches('_').to_string()
    }

    let name = match target {
        Target::Fn(item_fn) => item_fn.sig.ident.to_string(),
        Target::Struct(item_struct) => item_struct.ident.to_string(),
    };

    let type_name = sanitize_type_name(ty);

    /*
    The generated function name will be in the form `dilib_<name>_<return_type>`.
        Example: fn dilib_get_data_string() -> String
    */
    let name = format!("dilib_{}{}", name, type_name);
    syn::Ident::new(&name, Span::call_site())
}

// Providers codegen
fn get_injection_key(ty: &syn::Type, name: Option<&str>) -> TokenStream {
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

fn get_scoped_provider(item_fn: &ItemFn, bind: Option<&syn::Type>) -> TokenStream {
    let fn_name = item_fn.sig.ident.clone();

    if let Some(bind) = bind {
        let factory = quote! {
           || std::boxed::Box::<#bind>::new(#fn_name())
        };

        quote! {
            dilib::Provider::Scoped(
                dilib::Scoped::from_factory(#factory)
            )
        }
    } else {
        quote! {
            dilib::Provider::Scoped(
                dilib::Scoped::from_factory(#fn_name)
            )
        }
    }
}

fn get_resolved_scoped_provider(item_fn: &ItemFn, ty: &syn::Type, bind: Option<&syn::Type>) -> TokenStream {
    let fn_name = item_fn.sig.ident.clone();
    let resolved_args = ResolvedFnArg::from_fn(item_fn);
    let arg_names = resolved_args
        .iter()
        .map(|arg| syn::Ident::new(&arg.arg_name, Span::call_site()))
        .collect::<Vec<_>>();

    if let Some(bind) = bind {
        quote! {
            dilib::Provider::Scoped(
                dilib::Scoped::from_construct(|container: &dilib::Container| -> #bind {
                    #(#resolved_args)*
                    std::boxed::Box::new(#fn_name(#(#arg_names),*))
                })
            )
        }
    } else {
        quote! {
            dilib::Provider::Scoped(
                dilib::Scoped::from_construct(|container: &dilib::Container| -> #ty {
                    #(#resolved_args)*
                    #fn_name(#(#arg_names),*)
                })
            )
        }
    }
}

fn get_singleton_provider(item_fn: &ItemFn, bind: Option<&syn::Type>) -> TokenStream {
    let fn_name = item_fn.sig.ident.clone();
    let fn_call = quote! { #fn_name () };

    if let Some(bind) = bind {
        let val = quote! { std::boxed::Box::<#bind>::new(#fn_call()) };
        quote! {
            dilib::Provider::Singleton(
                dilib::Shared::new(#val)
            )
        }
    } else {
        quote! {
            dilib::Provider::Singleton(
                dilib::Shared::new(#fn_call)
            )
        }
    }
}

fn get_resolved_singleton_provider(item_fn: &ItemFn, ty: &syn::Type, bind: Option<&syn::Type>) -> TokenStream {
    let fn_name = item_fn.sig.ident.clone();
    let resolved_args = ResolvedFnArg::from_fn(item_fn);
    let arg_names = resolved_args
        .iter()
        .map(|arg| syn::Ident::new(&arg.arg_name, Span::call_site()))
        .collect::<Vec<_>>();

    if let Some(bind) = bind {
        quote! {
            dilib::Provider::Singleton(
                dilib::Shared::from_factory(|container: &dilib::Container| -> #bind {
                    #(#resolved_args)*
                    std::boxed::Box::new(#fn_name(#(#arg_names),*))
                })
            )
        }
    } else {
        quote! {
            dilib::Provider::Singleton(
                dilib::Shared::from_factory(|container: &dilib::Container| -> #ty {
                    #(#resolved_args)*
                    #fn_name(#(#arg_names),*)
                })
            )
        }
    }
}

fn get_inject_provider(item_struct: &ItemStruct, bind: Option<&syn::Type>) -> TokenStream {
    let struct_name = item_struct.ident.clone();

    if let Some(bind) = bind {
        quote! {
            dilib::Provider::Scoped(
                dilib::Scoped::from_construct(|container: &dilib::Container| -> #bind {
                        std::boxed::Box::new(<#struct_name as dilib::Inject> :: inject(container))
                    }
                )
            )
        }
    } else {
        quote! {
            dilib::Provider::Scoped(
                dilib::Scoped::from_construct(|container: &dilib::Container| -> #struct_name {
                        <#struct_name as dilib::Inject> :: inject(container)
                    }
                )
            )
        }
    }
}

fn get_singleton_inject_provider(item_struct: &ItemStruct, bind: Option<&syn::Type>) -> TokenStream {
    let struct_name = item_struct.ident.clone();

    if let Some(bind) = bind {
        quote! {
            dilib::Provider::Singleton(
                dilib::Shared::new_lazy(|container: &dilib::Container| -> #bind {
                        std::boxed::Box::new(<#struct_name as dilib::Inject> :: inject(container))
                    }
                )
            )
        }
    } else {
        quote! {
            dilib::Provider::Singleton(
                dilib::Shared::new_lazy(|container: &dilib::Container| -> #struct_name {
                        <#struct_name as dilib::Inject> :: inject(container)
                    }
                )
            )
        }
    }
}
