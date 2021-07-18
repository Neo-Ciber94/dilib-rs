use syn::{Type, Ident};
use syn::Lit;

pub struct Dependency {
    field: TargetField,
    field_type: Type,
    scope: Scope,
    name: Option<String>,
    default_value: Option<Lit>
}

impl Dependency {
    pub fn new(field: TargetField, field_type: Type, scope: Scope) -> Self {
        Dependency {
            field,
            field_type,
            scope,
            name: None,
            default_value: None
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_default_value(&mut self, default_value: Lit) {
        self.default_value = Some(default_value);
    }

    pub fn emit(self) -> proc_macro2::TokenStream {
        todo!()
    }
}

pub enum Scope {
    Scoped,
    Singleton
}

pub enum TargetField {
    Named(Ident),
    Unnamed(usize)
}