use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::Lit;
use syn::{Ident, Type};

#[derive(Debug)]
pub struct Dependency {
    field: TargetField,
    field_type: Type,
    scope: Scope,
    container: Ident,
    name: Option<String>,
    default_value: Option<DefaultValue>,
}

impl Dependency {
    pub fn new(field: TargetField, field_type: Type, scope: Scope, container: Ident) -> Self {
        Dependency {
            field,
            field_type,
            scope,
            container,
            name: None,
            default_value: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_scope(&mut self, scope: Scope) {
        self.scope = scope;
    }

    pub fn set_default_value(&mut self, default_value: DefaultValue) {
        self.default_value = Some(default_value);
    }

    pub fn var_name(&self) -> Ident {
        match self.field.clone() {
            TargetField::Named(s) => s,
            TargetField::Unnamed(n) => Ident::new(&format!("_{}", n), Span::call_site()),
        }
    }

    pub fn emit(&self) -> proc_macro2::TokenStream {
        let local_var = self.var_name();
        let var_type = self.field_type.clone();
        let expr = self.emit_assign_expr();
        let msg = self.get_error_message();

        let get_value = match &self.default_value {
            // let name : type = "string literal".into()
            Some(DefaultValue::Literal(Lit::Str(_))) => quote! { .into() },

            // let name : type = default.into()
            Some(DefaultValue::Literal(_)) => quote! { .into() },

            // let name : type = Default::default()
            Some(DefaultValue::Infer) => quote! {},

            // let name : type = container.get::<type>().expect("...")
            _ => quote! { .expect(#msg) },
        };

        match self.scope {
            Scope::Scoped => {
                quote! { let #local_var : #var_type = #expr #get_value ; }
            }
            Scope::Singleton => {
                quote! { let #local_var : dilib::Singleton< #var_type > = #expr #get_value ; }
            }
        }
    }

    fn get_error_message(&self) -> String {
        match (&self.scope, &self.name) {
            (Scope::Scoped, Some(name)) => {
                format!("cannot get scoped value of name \"{}\"", name)
            }
            (Scope::Singleton, Some(name)) => {
                format!("cannot get singleton value of name \"{}\"", name)
            }
            (Scope::Scoped, None) => {
                let ty = crate::helpers::token_stream_to_string_non_whitespace(
                    &self.field_type.to_token_stream(),
                );
                format!("cannot get scoped value of type `{}`", ty)
            }
            (Scope::Singleton, None) => {
                let ty = crate::helpers::token_stream_to_string_non_whitespace(
                    &self.field_type.to_token_stream(),
                );
                format!("cannot get singleton value of type `{}`", ty)
            }
        }
    }

    fn emit_assign_expr(&self) -> proc_macro2::TokenStream {
        if let Some(default_value) = &self.default_value {
            if self.name.is_some() {
                panic!("#[inject(default)] cannot contain a `name` field");
            }

            return match default_value {
                // let var : type = default_value
                DefaultValue::Literal(literal) => quote! { #literal },

                // let var : type = Default::default()
                DefaultValue::Infer => quote! { std::default::Default::default() },
            };
        }

        let container = &self.container;
        // let var_type = &self.field_type;

        match self.scope {
            Scope::Scoped => {
                if let Some(name) = self.name.as_deref() {
                    // let var : type = container.get_scoped_with_name(name);
                    quote! { #container.get_scoped_with_name(#name) }
                } else {
                    // let var : type = container.get_scoped();
                    quote! { #container.get_scoped() }
                }
            }
            Scope::Singleton => {
                if let Some(name) = self.name.as_deref() {
                    // let var : type = container.get_singleton_with_name(name);
                    quote! { #container.get_singleton_with_name(#name) }
                } else {
                    // let var : type = container.get_singleton();
                    quote! { #container.get_singleton() }
                }
            }
        }
    }
}

impl ToTokens for Dependency {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.emit())
    }
}

/// Source of the dependency value.
#[derive(Debug, Clone)]
pub enum Scope {
    Scoped,
    Singleton,
}

/// Default value of the dependency value.
#[derive(Debug, Clone)]
pub enum DefaultValue {
    /// A literal as default value.
    Literal(Lit),
    /// Infer the default value using `Default` trait.
    Infer,
}

#[derive(Debug, Clone)]
pub enum TargetField {
    Named(Ident),
    Unnamed(usize),
}
