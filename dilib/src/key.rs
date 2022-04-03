use std::any::TypeId;
use std::borrow::Cow;

/// Represents an unique key for value provider.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InjectionKey<'a> {
    type_id: TypeId,
    name: Option<Cow<'a, str>>,
}

impl<'a> InjectionKey<'a> {
    /// Constructs a new `InjectionKey`.
    ///
    /// # Params
    /// - `type_id`: The type of the value returned by the provider of this key.
    /// - `name`: The name of the key.
    pub fn new<S>(type_id: TypeId, name: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let name = name.map(|s| s.into());
        InjectionKey { type_id, name }
    }

    /// Constructs a new `InjectionKey` from the specified type `T`.
    pub fn of<T: ?Sized + 'static>() -> Self {
        Self::new::<String>(TypeId::of::<T>(), None)
    }

    /// Constructs a new `InjectionKey` from the specified type `T` and name.
    pub fn with_name<T: ?Sized + 'static>(name: &str) -> Self {
        Self::new(TypeId::of::<T>(), Some(name.to_string()))
    }

    /// Returns the `TypeId` of the value this key provider returns.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Returns the name of this key or `None`.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let type_id = TypeId::of::<i32>();
        let key = InjectionKey::new(type_id.clone(), Some("value"));

        assert_eq!(key.type_id(), type_id.clone());
        assert_eq!(key.name(), Some("value"));
    }

    #[test]
    fn of_test() {
        let key = InjectionKey::of::<String>();
        assert_eq!(key.type_id(), TypeId::of::<String>());
    }

    #[test]
    fn with_name_test() {
        let key = InjectionKey::with_name::<bool>("value2");

        assert_eq!(key.type_id(), TypeId::of::<bool>());
        assert_eq!(key.name(), Some("value2"));
    }
}
