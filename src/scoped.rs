use crate::Container;
use std::any::TypeId;
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Scoped {
    type_id: TypeId,
    inner: Inner,
}

impl Scoped {
    pub fn from_factory<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        let inner = Inner::from_factory(f);
        let type_id = TypeId::of::<T>();
        Scoped { type_id, inner }
    }

    pub fn from_injectable<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn(&Container) -> T + 'static,
    {
        let inner = Inner::from_injectable(f);
        let type_id = TypeId::of::<T>();
        Scoped { type_id, inner }
    }

    pub fn call_factory<T: 'static>(&self) -> Option<T> {
        if TypeId::of::<T>() != self.type_id {
            None
        } else {
            unsafe {
                let f = self.inner.as_factory()?;
                Some(f())
            }
        }
    }

    pub fn call_injectable<T: 'static>(&self, container: &Container) -> Option<T> {
        if TypeId::of::<T>() != self.type_id {
            None
        } else {
            unsafe {
                let f = self.inner.as_injectable()?;
                Some(f(container))
            }
        }
    }

    #[inline]
    pub fn is_factory(&self) -> bool {
        self.inner.is_factory()
    }

    #[inline]
    pub fn is_injectable(&self) -> bool {
        self.inner.is_injectable()
    }
}

#[derive(Debug, Clone)]
enum Inner {
    Factory(NonNull<dyn Fn()>),
    Injectable(NonNull<dyn Fn(&Container)>),
}

impl Inner {
    pub fn from_factory<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        let raw = Box::leak(Box::new(f));
        let ptr = raw as *mut dyn Fn() -> T as _;
        Inner::Factory(NonNull::new(ptr).unwrap())
    }

    pub fn from_injectable<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn(&Container) -> T + 'static,
    {
        let raw = Box::leak(Box::new(f));
        let ptr = raw as *mut dyn Fn(&Container) -> T as _;
        Inner::Injectable(NonNull::new(ptr).unwrap())
    }

    pub unsafe fn as_factory<T>(&self) -> Option<&dyn Fn() -> T> {
        match self {
            Inner::Injectable(_) => None,
            Inner::Factory(f) => {
                let ptr = f.as_ptr() as *mut dyn Fn() as *mut dyn Fn() -> T;
                Some(&*ptr)
            }
        }
    }

    pub unsafe fn as_injectable<T>(&self) -> Option<&dyn Fn(&Container) -> T> {
        match self {
            Inner::Factory(_) => None,
            Inner::Injectable(f) => {
                let ptr = f.as_ptr() as *mut dyn Fn() as *mut dyn Fn(&Container) -> T;
                Some(&*ptr)
            }
        }
    }

    pub fn is_factory(&self) -> bool {
        matches!(self, Inner::Factory(_))
    }

    pub fn is_injectable(&self) -> bool {
        matches!(self, Inner::Injectable(_))
    }
}
