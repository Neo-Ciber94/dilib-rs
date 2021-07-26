use std::str::FromStr;

use mattro::{MacroAttribute, MetaItem};
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    Data, DataStruct, DeriveInput, Field, Fields, GenericArgument, GenericParam, Generics, Ident,
    PathArguments, Type,
};

use crate::constructor::{TargetConstructor, TargetConstructorTokens};
use crate::dependency::{DefaultValue, Dependency, Scope, TargetField};
use crate::utils::InjectError;

#[derive(Debug)]
pub struct DeriveInjectable {
    target_type: Ident,
    container: Ident,
    constructor: Option<TargetConstructor>,
    deps: Vec<Dependency>,
    generics: Generics,
    is_unit: bool,
}

impl DeriveInjectable {
    pub fn new(
        target_type: Ident,
        container: Ident,
        constructor: Option<TargetConstructor>,
        deps: Vec<Dependency>,
        generics: Generics,
        is_unit: bool,
    ) -> Self {
        DeriveInjectable {
            target_type,
            container,
            constructor,
            deps,
            generics,
            is_unit,
        }
    }

    pub fn emit(&self) -> proc_macro2::TokenStream {
        let target_type = &self.target_type;

        if self.is_unit {
            return quote! {
                impl dilib::Injectable for #target_type {
                    fn resolve(_: &dilib::Container) -> Self {
                        #target_type
                    }
                }
            };
        }

        let container = &self.container;
        let deps = self.deps.as_slice();
        let generic_params = self.generics_params();
        let generic_types = self.generics_types();
        let where_clause = self.where_clause();

        let body = if let Some(constructor) = &self.constructor {
            let params = constructor
                .args
                .iter()
                .map(|s| Ident::new(s, Span::call_site()));

            // Type :: constructor ( params )
            let constructor_name = Ident::new(&constructor.name, Span::call_site());
            quote! { #target_type :: #constructor_name ( #(#params),* )}
        } else {
            let params = deps.iter().map(|s| s.var_name());

            // Type { params }
            quote! { #target_type { #(#params),* } }
        };

        quote! {
            impl #generic_params dilib::Injectable for #target_type #generic_types #where_clause {
                fn resolve(#container : &dilib::Container) -> Self {
                    #(#deps)*
                    #body
                }
            }
        }
    }

    // Generics with constrains: <T: Trait, B: OtherTrait>
    fn generics_params(&self) -> Option<proc_macro2::TokenStream> {
        if !self.generics.params.is_empty() {
            let params = &self.generics.params;
            Some(quote! { < #params > })
        } else {
            None
        }
    }

    // Generic types without contains: <T, B>
    fn generics_types(&self) -> Option<proc_macro2::TokenStream> {
        if !self.generics.params.is_empty() {
            let types = self
                .generics
                .params
                .iter()
                .filter_map(|param| match param {
                    GenericParam::Type(t) => Some(t.clone().ident),
                    GenericParam::Const(t) => Some(t.clone().ident),
                    GenericParam::Lifetime(_) => None,
                })
                .collect::<Vec<_>>();

            Some(quote! {
                < #(#types),* >
            })
        } else {
            None
        }
    }

    // Generics where clause: where T: Trait, B: OtherTrait
    fn where_clause(&self) -> Option<proc_macro2::TokenStream> {
        if self.generics.where_clause.is_some() {
            let tokens = self.generics.where_clause.to_token_stream();
            Some(tokens)
        } else {
            None
        }
    }
}

pub fn parse_derive_injectable(input: DeriveInput) -> DeriveInjectable {
    match &input.data {
        Data::Enum(_) => panic!("Enum types cannot implement `Injectable` with #[derive]"),
        Data::Union(_) => panic!("Union types cannot implement `Injectable` with #[derive]"),
        Data::Struct(data_struct) => {
            let target_type = input.ident.clone();
            let constructor = get_target_constructor(&input);
            let container = get_container_identifier(data_struct);
            let deps = get_deps(&data_struct.fields);
            let generics = input.generics.clone();
            let is_unit = data_struct.fields == Fields::Unit;

            DeriveInjectable::new(target_type, container, constructor, deps, generics, is_unit)
        }
    }
}

// #[inject(constructor="new(param1, param2, ...)")]
fn get_target_constructor(input: &DeriveInput) -> Option<TargetConstructor> {
    let attributes = input
        .attrs
        .iter()
        .cloned()
        .map(|a| MacroAttribute::new(a).unwrap())
        .filter(|a| a.path() == "inject")
        .collect::<Vec<_>>();

    // Check if the `#[inject(constructor="")]` are well formed
    for attr in &attributes {
        if attr.len() != 1 || attr.path() != crate::strings::INJECT {
            panic!("invalid inject constructor `{}`", attr);
        }
        if let Some(MetaItem::NameValue(s)) = attr.get(0) {
            if s.name != crate::strings::CONSTRUCTOR {
                panic!("invalid inject constructor `{}`", attr);
            }
        }
    }

    if let Some(attr) = attributes.last().cloned() {
        let name_value_attr = attr.into_name_values().unwrap();
        let value = name_value_attr.get("constructor").unwrap();
        let token_string = value.to_string_literal().unwrap();
        let tokens = proc_macro2::TokenStream::from_str(&token_string).unwrap();
        if let Ok(result) = syn::parse2::<TargetConstructorTokens>(tokens) {
            return Some(result.into_constructor());
        }
    }

    None
}

fn get_container_identifier(struct_data: &DataStruct) -> Ident {
    const CONTAINER_IDENT: &str = "container";

    match &struct_data.fields {
        Fields::Named(fields) => {
            let mut container_name = CONTAINER_IDENT.to_owned();
            let mut matches = 1_usize;

            for f in &fields.named {
                let field_name = f.ident.as_ref().unwrap().to_string();

                // We prevent name collisions
                // If there is a field named `container` we try other name
                // following the pattern: `container(N)` where N is the number of collisions
                if field_name == container_name {
                    container_name = format!("{}{}", container_name, matches);
                    matches += 1;
                }
            }

            Ident::new(container_name.as_str(), Span::call_site())
        }
        Fields::Unnamed(_) => Ident::new(CONTAINER_IDENT, Span::call_site()),
        Fields::Unit => Ident::new("_", Span::call_site()),
    }
}

fn get_deps(fields: &Fields) -> Vec<Dependency> {
    let mut deps = Vec::new();
    let container = Ident::new("container", Span::call_site());

    match fields {
        Fields::Unit => deps,
        Fields::Named(fields_named) => {
            for f in &fields_named.named {
                let field = TargetField::Named(f.ident.clone().unwrap());
                let (field_type, scope) = get_type_and_scope(&f.ty);
                let mut dependency = Dependency::new(field, field_type, scope, container.clone());
                set_dependency_attributes(f, &mut dependency);
                deps.push(dependency);
            }

            deps
        }
        Fields::Unnamed(fields_unnamed) => {
            for (index, f) in fields_unnamed.unnamed.iter().enumerate() {
                let field = TargetField::Unnamed(index);
                let (field_type, scope) = get_type_and_scope(&f.ty);
                let mut dependency = Dependency::new(field, field_type, scope, container.clone());
                set_dependency_attributes(f, &mut dependency);
                deps.push(dependency);
            }

            deps
        }
    }
}

fn get_type_and_scope(ty: &Type) -> (Type, Scope) {
    if let Some(generic) = get_singleton_type(ty) {
        (generic, Scope::Singleton)
    } else {
        (ty.clone(), Scope::Scoped)
    }
}

fn get_singleton_type(ty: &Type) -> Option<Type> {
    match ty {
        Type::Path(type_path) => {
            // Is declared as <T as Trait>::Inner
            if type_path.qself.is_some() {
                return None;
            }

            // SAFETY: A type path should have at least 1 element
            let segment = type_path.path.segments.last().unwrap();
            let ident = segment.ident.to_string();

            // Is `Singleton<T>`
            if ident == "Singleton" && !segment.arguments.is_empty() {
                match &segment.arguments {
                    PathArguments::AngleBracketed(bracketed) => {
                        let generic_arg = bracketed.args.first().unwrap();
                        if let GenericArgument::Type(Type::Path(generic_type)) = generic_arg {
                            return Some(Type::Path(generic_type.clone()));
                        }
                    }
                    _ => {}
                }
            }

            // Is `Arc<Mutex<T>>`
            if ident == "Arc" {
                match &segment.arguments {
                    PathArguments::AngleBracketed(bracket) => {
                        let generic_arg = bracket.args.first().unwrap();
                        if let GenericArgument::Type(Type::Path(generic)) = generic_arg {
                            let inner = generic.path.segments.last().unwrap();
                            if inner.ident.to_string() == "Mutex" {
                                return Some(Type::Path(generic.clone()));
                            }
                        }
                    }
                    _ => {}
                }
            }

            None
        }
        _ => None,
    }
}

fn set_dependency_attributes(field: &Field, dependency: &mut Dependency) {
    use crate::strings;

    let attributes = field
        .attrs
        .iter()
        .cloned()
        .map(|attr| MacroAttribute::new(attr).unwrap())
        .filter(|attr| attr.path() == strings::INJECT)
        .collect::<Vec<_>>();

    if attributes.len() > 0 {
        let attr = attributes.last().unwrap();

        match crate::utils::convert_to_inject_attribute_map(attr) {
            Ok(map) => {
                for (name, value) in map {
                    match name.as_str() {
                        strings::DEFAULT => {
                            if value.is_none() {
                                dependency.set_default_value(DefaultValue::Infer);
                            } else {
                                let lit = value.unwrap().as_literal().cloned().expect(&format!(
                                    "expected literal for default value: {}",
                                    attr
                                ));

                                dependency.set_default_value(DefaultValue::Literal(lit))
                            }
                        }
                        strings::NAME => {
                            let s = value.as_ref().cloned().unwrap().to_string_literal().expect(
                                &format!(
                                "expected string literal for `name` but was: `#[inject(name={})]`",
                                value.unwrap()
                            ),
                            );

                            dependency.set_name(s);
                        }
                        strings::SCOPE => {
                            let s = value.unwrap().to_string_literal().expect(&format!(
                                "expected string literal for scope: `#[inject(scope=\"...\")]`"
                            ));

                            let scope = match s.as_str() {
                                "singleton" => Scope::Singleton,
                                "scoped" => Scope::Scoped,
                                _ => panic!(
                                    "invalid scope value: `{}`, expected \"singleton\" or \"scoped\"", s
                                )
                            };

                            dependency.set_scope(scope);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Err(error) => panic_for_inject_error(error, attr),
        }
    }
}

fn panic_for_inject_error(error: InjectError, attr: &MacroAttribute) -> ! {
    match error {
        InjectError::DuplicatedKey(s) => {
            panic!("duplicated `#[inject]` key: {}", s);
        }
        InjectError::InvalidKey(s) => {
            // panic!(
            //     "invalid `#[inject]` key: `{}`, valid keys: `name`, `scope` and `default`",
            //     s
            // );

            panic!("invalid `#[inject]` key: `{}`", s);
        }
        InjectError::InvalidAttribute => {
            panic!("invalid attribute: {}", attr);
        }
    }
}
