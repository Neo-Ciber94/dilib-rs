use mattro::{MacroAttribute, MetaItem};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{ItemFn, PathArguments};

pub struct ResolvedFnArg {
    pub name: Option<String>,
    pub arg_name: String,
    pub ty: Box<syn::Type>,
}

impl ResolvedFnArg {
    pub fn from_fn(item_fn: &ItemFn) -> Vec<ResolvedFnArg> {
        let sig = &item_fn.sig;
        let mut args = Vec::new();

        for arg in sig.inputs.iter() {
            let arg_name = match arg {
                syn::FnArg::Typed(syn::PatType { pat, .. }) => match pat.as_ref() {
                    syn::Pat::Ident(syn::PatIdent { ref ident, .. }) => ident.to_string(),
                    _ => panic!("unexpected pattern in function argument"),
                },
                _ => panic!("expected named argument"),
            };

            let ty = match arg {
                syn::FnArg::Typed(syn::PatType { ty, .. }) => ty.clone(),
                _ => panic!("expected named argument"),
            };

            let name = get_inject_name(&arg_name, item_fn);

            args.push(ResolvedFnArg {
                name,
                arg_name,
                ty,
            });
        }

        args
    }
}

impl ToTokens for ResolvedFnArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ResolvedFnArg { name, arg_name, ty } = self;

        let get_provider = match (is_singleton(&ty), name) {
            (true, Some(name)) => {
                let inner = get_singleton_inner_type(&ty);
                quote! { get_singleton_with_name::<#inner>(#name) }
            },
            (true, None) => {
                let inner = get_singleton_inner_type(&ty);
                quote! { get_singleton::<#inner>() }
            },
            (false, Some(name)) => quote! { get_scoped_with_name::<#ty>(#name) },
            (false, None) => quote! { get_scoped::<#ty>() },
        };

        let arg_name = syn::Ident::new(&arg_name, proc_macro2::Span::call_site());
        let type_name = if is_singleton(ty) {
            format_tokens(&get_singleton_inner_type(&ty))
        } else {
            format_tokens(ty)
        };

        let result = quote! {
            let #arg_name = container . #get_provider .expect(concat!("failed to resolve: ", #type_name));
        };

        tokens.append_all(result);
    }
}

fn get_singleton_inner_type(ty: &syn::Type) -> Box<syn::Type> {
    match ty {
        syn::Type::Path(syn::TypePath { path, ..}) => {
            let last = path.segments.last().expect("expected generic type");
            match &last.arguments {
                PathArguments::AngleBracketed(angle_bracketed) => {
                    let args = angle_bracketed.args.iter().collect::<Vec<_>>();

                    if args.len() != 1 {
                        panic!("expected 'Singleton<T>' or 'Arc<T>' type");
                    }

                    match args.first().unwrap() {
                        syn::GenericArgument::Type(ty) => Box::new(ty.clone()),
                        _ => panic!("expected generic type"),
                    }
                }
                _ => panic!("expected generic type"),
            }
        },
        _ => panic!("expected 'Singleton<T>' or 'Ar<T>' type")
    }
}

fn is_singleton(ty: &syn::Type) -> bool {
    fn is_singleton_internal(path: &[String]) -> bool {
        let path_str = path.join("::");
        match path_str.as_str() {
            "Singleton" | "Arc" | "dilib::Singleton" | "std::sync::Arc" | "sync::Arc" => true,
            _ => false,
        }
    }

    match ty {
        syn::Type::Path(syn::TypePath { path, .. }) => {
            let segments = path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>();

            is_singleton_internal(&segments)
        }
        _ => false,
    }
}

#[cfg(feature="when macro attribute in fn args get stabilized")]
fn get_inject_name(attr: &[Attribute]) -> Option<String> {
    let attributes = attr
        .iter()
        .map(|attr| MacroAttribute::new(attr.clone()).ok())
        .filter_map(|attr| attr)
        .collect::<Vec<_>>();

    let inject_attr = attributes
        .into_iter()
        .find(|attr| attr.path() == "inject");

    match inject_attr {
        Some(attr) => {
            let name_value_attr = attr.into_name_values().ok()?;
            let mut map = name_value_attr.into_iter()
                .collect::<HashMap<String, _>>();

            let name = map.remove_entry("name")
                .map(|(_, value)| value.to_string_literal().expect("expected #[inject] 'name' to be a string"));

            // The rest are unknown
            if let Some((key, _)) = map.iter().next() {
                panic!("unknown key for #[inject]: {}", key);
            }

            name
        }
        None => None,
    }
}

fn get_inject_name(arg_name: &str, item_fn: &ItemFn) -> Option<String> {
    const INVALID_SIGNATURE : &str = "invalid #[inject] signature, expected: #[inject(arg, name = \"value\")]";

    let attrs = item_fn.attrs.iter()
        .cloned()
        .map(|att| MacroAttribute::new(att).ok())
        .filter_map(|att| att)
        .filter(|att| att.path() == "inject")
        .collect::<Vec<_>>();

    if attrs.is_empty() {
        return None;
    }

    let mut inject_name: Option<String> = None;

    for attr in attrs {
        let arg = match attr.get(0) {
            Some(MetaItem::Path(path)) => { path.clone()}
            _ => panic!("{}", INVALID_SIGNATURE),
        };

        if arg != arg_name {
            continue;
        }

        inject_name = match attr.get(1) {
            Some(MetaItem::NameValue(name_value)) => {
                let name = &name_value.name;
                let value = &name_value.value;

                if name != "name" {
                    panic!("{}", INVALID_SIGNATURE);
                }

                match value.to_string_literal() {
                    Some(value) => Some(value),
                    None => panic!("{}", INVALID_SIGNATURE),
                }
            }
            _ => panic!("{}", INVALID_SIGNATURE),
        };
    }

    if inject_name.is_none() {
        panic!("unable to find '{0}' for '#[inject({0}, ...)]'", arg_name);
    }

    inject_name
}

fn format_tokens<T>(tokens: &T) -> String where T: ToTokens {
    tokens.to_token_stream()
        .into_iter()
        .flat_map(|t| t.to_string().chars().collect::<Vec<char>>())
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
}