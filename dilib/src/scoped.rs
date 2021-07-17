use crate::Container;
use std::any::TypeId;
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Scoped {
    type_id: TypeId,
    inner: BoxClosure,
}

impl Scoped {
    pub fn from_factory<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        let inner = BoxClosure::from_fn(f);
        let type_id = TypeId::of::<T>();
        Scoped { type_id, inner }
    }

    pub fn from_injectable<T, F>(f: F) -> Self
    where
        T: 'static,
        F: Fn(&Container) -> T + 'static,
    {
        let inner = BoxClosure::from_fn_arg(f);
        let type_id = TypeId::of::<T>();
        Scoped { type_id, inner }
    }

    pub fn call_factory<T: 'static>(&self) -> Option<T> {
        if TypeId::of::<T>() != self.type_id {
            None
        } else {
            unsafe {
                let f = self.inner.as_fn()?;
                Some(f())
            }
        }
    }

    pub fn call_injectable<T: 'static>(&self, container: &Container) -> Option<T> {
        if TypeId::of::<T>() != self.type_id {
            None
        } else {
            unsafe {
                let f = self.inner.as_fn_arg()?;
                Some(f(container))
            }
        }
    }

    #[inline]
    pub fn is_factory(&self) -> bool {
        !self.inner.takes_args()
    }

    #[inline]
    pub fn is_injectable(&self) -> bool {
        self.inner.takes_args()
    }
}


#[derive(Debug, Clone)]
enum BoxClosure {
    Fn(NonNull<dyn Fn()>),
    FnArg(NonNull<dyn Fn()>),
}

impl BoxClosure {
    pub fn from_fn<T, F>(f: F) -> Self
        where
            T: 'static,
            F: Fn() -> T + 'static,
    {
        let raw = Box::leak(Box::new(f));
        let ptr = raw as *mut dyn Fn() -> T as _;
        BoxClosure::Fn(NonNull::new(ptr).unwrap())
    }

    pub fn from_fn_arg<T, A, F>(f: F) -> Self
        where
            T: 'static,
            A: 'static,
            F: Fn(A) -> T + 'static,
    {
        let raw = Box::leak(Box::new(f));
        let ptr = raw as *mut dyn Fn(A) -> T as _;
        BoxClosure::FnArg(NonNull::new(ptr).unwrap())
    }

    pub unsafe fn as_fn<T>(&self) -> Option<&dyn Fn() -> T> {
        match self {
            BoxClosure::FnArg(_) => None,
            BoxClosure::Fn(f) => {
                let ptr = f.as_ptr() as *mut dyn Fn() as *mut dyn Fn() -> T;
                Some(&*ptr)
            }
        }
    }

    pub unsafe fn as_fn_arg<T, Arg>(&self) -> Option<&dyn Fn(Arg) -> T> {
        match self {
            BoxClosure::Fn(_) => None,
            BoxClosure::FnArg(f) => {
                let ptr = f.as_ptr() as *mut dyn Fn() as *mut dyn Fn(Arg) -> T;
                Some(&*ptr)
            }
        }
    }

    pub fn takes_args(&self) -> bool {
        matches!(self, BoxClosure::FnArg(_))
    }
}

impl Drop for BoxClosure {
    fn drop(&mut self) {
        unsafe {
            match self {
                BoxClosure::FnArg(x) |
                BoxClosure::Fn(x) => {
                    let raw = x.as_ptr();
                    let _ = Box::from_raw(raw);
                }
            }
        }
    }
}