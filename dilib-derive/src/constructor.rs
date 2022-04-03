use proc_macro2::Ident;
use syn::parenthesized;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Paren};

#[derive(Debug)]
pub struct TargetConstructor {
    pub(crate) name: String,
    pub(crate) args: Vec<String>,
}

impl TargetConstructor {
    pub fn new(name: String, args: Vec<String>) -> Self {
        TargetConstructor { name, args }
    }
}

// new(param1, param2, ...)
#[derive(Debug)]
pub struct TargetConstructorTokens {
    method: Ident,
    #[allow(dead_code)]
    paren_token: Paren,
    args: Punctuated<Ident, Comma>,
}

impl TargetConstructorTokens {
    pub fn into_constructor(self) -> TargetConstructor {
        let name = self.method.to_string();
        let args = self.args.iter().map(|s| s.to_string()).collect::<Vec<_>>();

        TargetConstructor::new(name, args)
    }
}

impl Parse for TargetConstructorTokens {
    #[allow(clippy::eval_order_dependence)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(TargetConstructorTokens {
            method: input.parse()?,
            paren_token: parenthesized!(content in input),
            args: content.parse_terminated(Ident::parse)?,
        })
    }
}
