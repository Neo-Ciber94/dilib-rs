use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Error, ItemFn, ItemStruct, TypePath};

#[derive(Debug, Clone)]
pub enum Target {
    Fn(ItemFn),
    Struct(ItemStruct),
}

impl Target {
    pub fn target_type(&self) -> Box<syn::Type> {
        match &self {
            Target::Fn(item_fn) => match item_fn.sig.output.clone() {
                syn::ReturnType::Default => Box::new(syn::parse_quote! { () }),
                syn::ReturnType::Type(_, ty) => ty,
            },
            Target::Struct(s) => {
                let ident = s.ident.clone();
                let path = TypePath {
                    qself: None,
                    path: syn::Path::from(ident),
                };
                let ty = syn::Type::Path(path);
                Box::new(ty)
            }
        }
    }
}

impl Parse for Target {
    // There is not actually a double ref
    #[allow(clippy::clone_double_ref)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // TODO: Optimize with lookahead
        if let Ok(item_fn) = input.clone().parse() {
            return Ok(Target::Fn(item_fn));
        }

        if let Ok(item_struct) = input.clone().parse() {
            return Ok(Target::Struct(item_struct));
        }

        Err(Error::new(
            proc_macro2::Span::call_site(),
            "Expected a function or struct",
        ))
    }
}

impl ToTokens for Target {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Target::Fn(item_fn) => item_fn.to_tokens(tokens),
            Target::Struct(item_struct) => item_struct.to_tokens(tokens),
        }
    }
}
