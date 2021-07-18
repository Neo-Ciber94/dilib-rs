use syn::{Type, Ident, DeriveInput, Data, Fields};
use quote::quote;
use crate::dependency::{Dependency, TargetField, Scope};
use proc_macro::TokenStream;

pub struct InjectableTarget {
    target_type: Ident,
    constructor: Option<TargetConstructor>,
    deps: Vec<Dependency>,
}

pub struct TargetConstructor {
    name: String,
    params: Vec<String>
}

impl InjectableTarget {
    pub fn new(target_type: Ident, constructor: Option<TargetConstructor>, deps: Vec<Dependency>) -> Self {
        InjectableTarget {
            target_type,
            constructor,
            deps,
        }
    }

    pub fn emit(self) -> proc_macro2::TokenStream {
        let s = quote! {};
        s
    }
}

pub fn parse_derive_injectable(input: DeriveInput) -> InjectableTarget {
    match &input.data {
        Data::Enum(_) => panic!("Enum types are not supported"),
        Data::Union(_) => panic!("Union types are not supported"),
        Data::Struct(data_struct) => {
            let target_type = input.ident.clone();
            let constructor = get_target_constructor(&input);
            let deps = get_deps(&data_struct.fields);

            InjectableTarget::new(target_type, constructor, deps)
        }
    }
}

fn get_target_constructor(_input: &DeriveInput) -> Option<TargetConstructor> {
    None
}

fn get_deps(fields: &Fields) -> Vec<Dependency> {
    let mut deps = Vec::new();

    // todo: check for attributes for additional config

    match fields {
        Fields::Unit => deps,
        Fields::Named(fields_named) => {
            for f in &fields_named.named {
                let field = TargetField::Named(f.ident.clone().unwrap());
                let field_type = f.ty.clone();
                let scope = Scope::Scoped;
                let mut dependency = Dependency::new(field, field_type, scope);

                deps.push(dependency);
            }

            deps
        },
        Fields::Unnamed(fields_unnamed) => {
            for (index, f) in fields_unnamed.unnamed.iter().enumerate() {
                let field = TargetField::Unnamed(index);
                let field_type = f.ty.clone();
                let scope = Scope::Scoped;
                let mut dependency = Dependency::new(field, field_type, scope);

                deps.push(dependency);
            }

            deps
        },
    }
}