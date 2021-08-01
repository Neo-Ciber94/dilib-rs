use crate::provider::ProviderKind;
use std::any::TypeId;
use std::borrow::Cow;

/// Represents an unique key for value provider.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InjectionKey<'a> {
    type_id: TypeId,
    name: Option<Cow<'a, str>>,
    kind: ProviderKind,
}

impl<'a> InjectionKey<'a> {
    /// Constructs a new `InjectionKey`.
    ///
    /// # Params
    /// - `type_id`: The type of the value returned by the provider of this key.
    /// - `kind`: The type of provider.
    /// - `name`: The name of the key.
    pub fn new<S>(type_id: TypeId, kind: ProviderKind, name: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let name = name.map(|s| s.into());
        InjectionKey {
            type_id,
            kind,
            name,
        }
    }

    /// Constructs a new `InjectionKey` from the specified type `T` and provider kind.
    pub fn of<T: ?Sized + 'static>(kind: ProviderKind) -> Self {
        Self::new::<String>(TypeId::of::<T>(), kind, None)
    }

    /// Constructs a new `InjectionKey` from the specified type `T`, provider kind and name.
    pub fn with_name<T: ?Sized + 'static>(kind: ProviderKind, name: &str) -> Self {
        Self::new(TypeId::of::<T>(), kind, Some(name.to_string()))
    }

    /// Returns the `TypeId` of the value this key provider returns.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Returns the name of this key or `None`.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the kind of this key provider.
    pub fn kind(&self) -> ProviderKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let type_id = TypeId::of::<i32>();
        let key = InjectionKey::new(type_id.clone(), ProviderKind::Scoped, Some("value"));

        assert_eq!(key.type_id(), type_id.clone());
        assert_eq!(key.kind(), ProviderKind::Scoped);
        assert_eq!(key.name(), Some("value"));
    }

    #[test]
    fn of_test() {
        let key = InjectionKey::of::<String>(ProviderKind::Singleton);

        assert_eq!(key.type_id(), TypeId::of::<String>());
        assert_eq!(key.kind(), ProviderKind::Singleton);
    }

    #[test]
    fn with_name_test() {
        let key = InjectionKey::with_name::<bool>(ProviderKind::Singleton, "value2");

        assert_eq!(key.type_id(), TypeId::of::<bool>());
        assert_eq!(key.kind(), ProviderKind::Singleton);
        assert_eq!(key.name(), Some("value2"));
    }
}