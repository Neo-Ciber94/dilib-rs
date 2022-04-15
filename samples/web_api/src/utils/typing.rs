// Copied from: dilib/z_typing.rs
#![allow(dead_code)]
use std::any::TypeId;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Represents a type.
#[derive(Debug, Clone, Copy)]
pub struct Type {
    id: TypeId,
    name: &'static str,
}

impl Type {
    /// Returns the `Type` for the given type.
    pub fn of<T: 'static>() -> Self {
        let id = TypeId::of::<T>();
        let name = std::any::type_name::<T>();
        Type { id, name }
    }

    /// Returns the `TypeId` of the type.
    #[inline]
    pub fn id(&self) -> TypeId {
        self.id
    }

    /// Returns the name of the type.
    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Type {}

impl PartialEq<Self> for Type {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd<Self> for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

#[cfg(test)]
mod tests {
    use super::Type;
    use std::any::TypeId;

    #[test]
    fn type_and_id_test() {
        let ty = Type::of::<String>();

        assert_eq!(ty.id(), TypeId::of::<String>());
        assert!(ty.name().contains("String"));
    }
}
