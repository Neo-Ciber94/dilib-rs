use std::fmt::{Display, Formatter, Debug};
use mattro::{Value, NameValue};

pub struct Error {
    inner: Inner
}

impl Error {
    pub fn new<E: Into<Box<dyn std::error::Error + Send + Sync>>>(kind: ErrorKind, error: E) -> Self {
        Error {
            inner: Inner::Custom(Custom {
                kind,
                error: error.into()
            })
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        match &self.inner {
            Inner::Simple(kind) => kind,
            Inner::Custom(custom) => &custom.kind
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error {
            inner: Inner::Simple(kind)
        }
    }
}

enum Inner {
    Simple(ErrorKind),
    Custom(Custom)
}

#[derive(Eq, PartialEq)]
pub enum ErrorKind {
    /// Invalid `#[inject]` attribute.
    InvalidAttribute,
    /// Duplicated name.
    DuplicatedKey(String),
    /// Invalid name value.
    InvalidNameValue(NameValue),
    /// Unknown key.
    UnknownKey(String),
    /// Invalid default value.
    InvalidDefaultValue(NameValue),
    /// Invalid scope kind value.
    InvalidScopeKind(String),
    /// Unknown error.
    Other(String)
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<dyn std::error::Error + Send + Sync>
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.inner {
            Inner::Simple(_) => None,
            Inner::Custom(custom) => Some(&*custom.error)
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Debug for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidAttribute => write!(f, "invalid `#[inject]` attribute format"),
            ErrorKind::DuplicatedKey(s) => write!(f, "duplicated `#[inject]` key: {}", s),
            ErrorKind::InvalidNameValue(n) => write!(f, "invalid `#[inject]` name-value: `{}`", n),
            ErrorKind::UnknownKey(s) => write!(f, "unknown #[inject] key: `{}`", s),
            ErrorKind::InvalidDefaultValue(n) => write!(f, "invalid `#[inject({})]`", n),
            ErrorKind::InvalidScopeKind(s) => write!(f, "invalid `#[inject]` scope kind: `{}`", s),
            ErrorKind::Other(s) => write!(f, "{}", s)
        }
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Inner::Simple(kind) => write!(f, "{}", kind),
            Inner::Custom(custom) => write!(f, "{}: {}", custom.kind, custom.error)
        }
    }
}