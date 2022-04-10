use std::ops::Deref;
use std::sync::Arc;

/// Represents a value from a [`Container`].
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Resolved<T> {
    /// An owned value.
    Scoped(T),
    /// A shared value.
    Singleton(Arc<T>),
}

impl<T> Resolved<T> {
    /// Returns `true` if the value is a singleton.
    pub fn is_singleton(&self) -> bool {
        matches!(self, Resolved::Singleton(_))
    }

    /// Returns `true` if the value is scoped.
    pub fn is_scoped(&self) -> bool {
        matches!(self, Resolved::Scoped(_))
    }

    /// Returns a mutable reference to the value.
    ///
    /// # Returns
    /// - Some(&mut T): If the value is scoped or if the singleton has only has one reference to it.
    /// - None: If the value is a singleton and have more than a reference to it.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        match self {
            Resolved::Scoped(ref mut v) => Some(v),
            Resolved::Singleton(ref mut v) => Arc::get_mut(v),
        }
    }

    /// Attempts to downcast the value to a singleton.
    pub fn into_singleton(self) -> Option<Arc<T>> {
        match self {
            Resolved::Singleton(t) => Some(t),
            _ => None,
        }
    }

    /// Attempts to downcast the value to a value.
    pub fn into_scoped(self) -> Option<T> {
        match self {
            Resolved::Scoped(t) => Some(t),
            _ => None,
        }
    }
}

impl<T: Clone> Resolved<T> {
    /// Returns a copy of the value.
    pub fn cloned(&self) -> T {
        match self {
            Resolved::Scoped(v) => v.clone(),
            Resolved::Singleton(v) => v.as_ref().clone(),
        }
    }
}

impl<T> Deref for Resolved<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Resolved::Scoped(t) => t,
            Resolved::Singleton(t) => t,
        }
    }
}

impl<T> AsRef<T> for Resolved<T> {
    fn as_ref(&self) -> &T {
        match self {
            Resolved::Scoped(t) => t,
            Resolved::Singleton(t) => t.deref(),
        }
    }
}
