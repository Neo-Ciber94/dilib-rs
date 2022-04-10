use std::any::TypeId;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Error returned when a dependency cannot be found.
#[derive(Debug, Clone)]
pub struct MissingDependency {
    ty: Type,
    name: Option<String>
}

#[derive(Debug, Clone, Copy)]
pub struct Type {
    id: TypeId,
    name: &'static str
}

impl Type {
    pub fn of<T: 'static>() -> Self {
        let id = TypeId::of::<T>();
        let name = std::any::type_name::<T>();
        Type { id, name }
    }

    #[inline]
    pub fn id(&self) -> TypeId {
        self.id
    }

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