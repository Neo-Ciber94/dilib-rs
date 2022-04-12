// This module is not TRACKED and may be removed

use crate::typing::Type;
use std::error;
use std::fmt::{Debug, Display, Formatter};

/// Error returned when a dependency cannot be found.
#[derive(Debug, Clone)]
pub struct MissingDependency {
    ty: Type,
    name: Option<String>,
}

/// Represents an error occurring when resolving a dependency from the `Container`.
pub struct ResolveError {
    inner: Inner,
}

enum Inner {
    MissingDependency(MissingDependency),
    Custom(Box<dyn error::Error + Send + Sync>),
    Simple(SimpleMessage),
}

struct SimpleMessage {
    message: &'static str,
    description: Option<String>,
}

impl ResolveError {
    /// Constructs a new `ResolveError` from an error.
    pub fn new<E>(error: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync + 'static>>,
    {
        ResolveError {
            inner: Inner::Custom(error.into()),
        }
    }
    /// Constructs a new `ResolveError` for a dependency missing.
    pub fn missing_dependency<T>() -> Self
    where
        T: 'static,
    {
        ResolveError {
            inner: Inner::MissingDependency(MissingDependency {
                ty: Type::of::<T>(),
                name: None,
            }),
        }
    }

    /// Constructs a new `ResolveError` for a dependency with a name missing.
    pub fn missing_dependency_with_name<T, S>(name: S) -> Self
    where
        T: 'static,
        S: Into<String>,
    {
        ResolveError {
            inner: Inner::MissingDependency(MissingDependency {
                ty: Type::of::<T>(),
                name: Some(name.into()),
            }),
        }
    }

    /// Constructs a new `ResolveError` with a message.
    pub fn with_message(message: &'static str) -> Self {
        ResolveError {
            inner: Inner::Simple(SimpleMessage {
                message,
                description: None,
            }),
        }
    }

    /// Constructs a new `ResolveError` with a message and a description.
    pub fn with_description(message: &'static str, description: String) -> Self {
        ResolveError {
            inner: Inner::Simple(SimpleMessage {
                message,
                description: Some(description),
            }),
        }
    }
}

impl Debug for ResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            Inner::MissingDependency(deps) => {
                write!(f, "missing dependency")?;
                if let Some(ref name) = deps.name {
                    write!(f, " (name: {})", name)?;
                }

                write!(f, " :of type {}", deps.ty.name())
            }
            Inner::Custom(custom) => {
                write!(f, "{}", custom.to_string())
            }
            Inner::Simple(simple) => {
                write!(f, "{}", simple.message)?;
                if let Some(ref description) = simple.description {
                    write!(f, ": {}", description)?;
                }
                Ok(())
            }
        }
    }
}

impl Display for ResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl error::Error for ResolveError {}
