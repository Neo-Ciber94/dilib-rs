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

#[derive(Debug, Eq, PartialEq)]
pub enum StructKind {
    // `MyStruct` or `MyStruct{}`
    Unit,

    // MyStruct { a: i32, b: String }
    Named,

    // MyStruct(i32, String)
    Tuple,
}

#[derive(Debug)]
pub struct DeriveInject {
    target_type: Ident,
    container: Ident,
    constructor: Option<TargetConstructor>,
    deps: Vec<Dependency>,
    generics: Generics,
    kind: StructKind,
}

impl DeriveInject {
    pub fn new(
        target_type: Ident,
        container: Ident,
        constructor: Option<TargetConstructor>,
        deps: Vec<Dependency>,
        generics: Generics,
        kind: StructKind,
    ) -> Self {
        DeriveInject {
            target_type,
            container,
            constructor,
            deps,
            generics,
            kind,
        }
    }

    pub fn expand(&self) -> proc_macro2::TokenStream {
        let target_type = &self.target_type;
        let container = &self.container;
        let deps = self.deps.as_slice();
        let generic_params = self.generics_params();
        let generic_types = self.generics_types();
        let where_clause = self.where_clause();

        let body = if self.kind == StructKind::Unit {
            quote! { #target_type }
        } else if let Some(constructor) = &self.constructor {
            let params = constructor
                .args
                .iter()
                .map(|s| Ident::new(s, Span::call_site()));

            // Type :: constructor ( params )
            let constructor_name = Ident::new(&constructor.name, Span::call_site());
            quote! { #target_type :: #constructor_name ( #(#params),* )}
        } else {
            let params = deps.iter().map(|s| s.var_name());

            match self.kind {
                StructKind::Named => {
                    // Type { params }
                    quote! { #target_type { #(#params),* } }
                }
                StructKind::Tuple => {
                    // Type ( params )
                    quote! { #target_type ( #(#params),* ) }
                }
                StructKind::Unit => unreachable!(),
            }
        };

        quote! {
            impl #generic_params dilib::Inject for #target_type #generic_types #where_clause {
                #[allow(unused)]
                #[allow(dead_code)]
                fn inject(#container : &dilib::Container) -> Self {
                    #(#deps)*
                    #body
                }
            }
        }
    }

    // Generics for: impl<A, B, C>
    fn generics_params(&self) -> Option<proc_macro2::TokenStream> {
        if !self.generics.params.is_empty() {
            let params = self.generics.params.iter().cloned().map(|t| {
                match t {
                    // We remove `<T = type>` which is not allowed in `impl<A, B, C>`
                    GenericParam::Type(mut ty) => {
                        ty.eq_token = None;
                        ty.default = None;
                        GenericParam::Type(ty)
                    }
                    _ => t,
                }
            });

            Some(quote! { < #(#params),* > })
        } else {
            None
        }
    }

    // Generic types for: MyStruct<A, B, C>
    fn generics_types(&self) -> Option<proc_macro2::TokenStream> {
        if !self.generics.params.is_empty() {
            let types = self
                .generics
                .params
                .iter()
                .map(|param| match param {
                    GenericParam::Type(t) => t.ident.to_token_stream(),
                    GenericParam::Const(t) => t.ident.to_token_stream(),
                    GenericParam::Lifetime(t) => t.to_token_stream(),
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

pub fn parse_derive_inject(input: DeriveInput) -> DeriveInject {
    match &input.data {
        Data::Enum(_) => panic!("Enum types cannot implement `Inject` with #[derive]"),
        Data::Union(_) => panic!("Union types cannot implement `Inject` with #[derive]"),
        Data::Struct(data_struct) => {
            let target_type = input.ident.clone();
            let constructor = get_target_constructor(&input);
            let container = get_container_identifier(data_struct);
            let deps = get_deps(&data_struct.fields);
            let generics = input.generics.clone();
            let kind = get_struct_kind(data_struct);

            DeriveInject::new(target_type, container, constructor, deps, generics, kind)
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

            // Is `Singleton<T>` or `Arc<T>`
            if (ident == "Singleton" || ident == "Arc") && !segment.arguments.is_empty() {
                if let PathArguments::AngleBracketed(bracketed) = &segment.arguments {
                    let generic_arg = bracketed.args.first().unwrap();
                    if let GenericArgument::Type(Type::Path(generic_type)) = generic_arg {
                        return Some(Type::Path(generic_type.clone()));
                    }
                }
            }

            None
        }
        _ => None,
    }
}

fn get_struct_kind(data_struct: &DataStruct) -> StructKind {
    match data_struct.fields {
        Fields::Named(_) => StructKind::Named,
        Fields::Unnamed(_) => StructKind::Tuple,
        Fields::Unit => StructKind::Unit,
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

    if !attributes.is_empty() {
        let attr = attributes.last().unwrap();

        match crate::utils::convert_to_inject_attribute_map(attr) {
            Ok(map) => {
                for (name, value) in map {
                    match name.as_str() {
                        strings::DEFAULT => {
                            if let Some(default_value) = value {
                                let lit =
                                    default_value.as_literal().cloned().unwrap_or_else(|| {
                                        panic!("expected literal for default value: {}", attr)
                                    });

                                dependency.set_default_value(DefaultValue::Literal(lit))
                            } else {
                                dependency.set_default_value(DefaultValue::Infer);
                            }
                        }
                        strings::NAME => {
                            let s = value
                                .as_ref()
                                .cloned()
                                .unwrap()
                                .to_string_literal()
                                .unwrap_or_else(|| {
                                    panic!(
                                "expected string literal for `name` but was: `#[inject(name={})]`",
                                value.unwrap()
                            )
                                });

                            dependency.set_name(s);
                        }
                        strings::SCOPE => {
                            let s = value.unwrap().to_string_literal().expect(
                                "expected string literal for scope: `#[inject(scope=\"...\")]`",
                            );

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
