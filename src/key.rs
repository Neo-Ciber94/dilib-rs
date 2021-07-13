use std::any::TypeId;
use std::borrow::Cow;
use crate::provider::ProviderKind;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InjectionKey<'a> {
    type_id: TypeId,
    name: Option<Cow<'a, str>>,
    kind: ProviderKind
}

impl<'a> InjectionKey<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(type_id: TypeId, kind: ProviderKind, name: Option<S>) -> Self {
        let name = name.map(|s| s.into());
        InjectionKey { type_id, kind, name }
    }

    pub fn of<T: 'static>(kind: ProviderKind) -> Self {
        Self::new::<String>(TypeId::of::<T>(), kind, None)
    }

    pub fn with_name<T: 'static>(kind: ProviderKind, name: &str) -> Self {
        Self::new(TypeId::of::<T>(), kind, Some(name.to_string()))
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn kind(&self) -> ProviderKind {
        self.kind
    }
}