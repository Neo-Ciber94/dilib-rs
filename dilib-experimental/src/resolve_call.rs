use dilib::Container;

/// Provides a way to call a function using a `Container` dependencies.
pub trait ResolveCall<Args> {
    type Output;
    fn resolve_call(&self, container: &Container) -> Self::Output;
}

/// Provides a way to call a mut function using a `Container` dependencies.
pub trait ResolveCallMut<Args> {
    type Output;
    fn resolve_call_mut(&mut self, container: &Container) -> Self::Output;
}

macro_rules! impl_resolve_call_fn {
    ($($t:ident),+) => {
        impl<Function, Out, $($t),+> ResolveCall<($($t),+,)> for Function
         where Function: Fn($(&$t),+) -> Out,
         $($t: Send + Sync + 'static),+ {
            type Output = Out;
            fn resolve_call(&self, container: &Container) -> Self::Output {
                (self)(
                    $(
                        container.get::<$t>()
                            .as_ref()
                            .unwrap_or_else(|| panic!("unable to get {}", std::any::type_name::<$t>()))
                    ),+
                )
            }
        }
    };

    (mut $($t:ident),+) => {
        impl<Function, Out, $($t),+> ResolveCallMut<($($t),+,)> for Function
         where Function: FnMut($(&$t),+) -> Out,
         $($t: Send + Sync + 'static),+ {
            type Output = Out;
            fn resolve_call_mut(&mut self, container: &Container) -> Self::Output {
                (self)(
                    $(
                        container.get::<$t>()
                            .as_ref()
                            .unwrap_or_else(|| panic!("unable to get '{}'", std::any::type_name::<$t>()))
                    ),+
                )
            }
        }
    };
}

// Only implemented until 12: https://doc.rust-lang.org/std/primitive.tuple.html#trait-implementations-1

impl_resolve_call_fn!(T0);
impl_resolve_call_fn!(T0, T1);
impl_resolve_call_fn!(T0, T1, T2);
impl_resolve_call_fn!(T0, T1, T2, T3);
impl_resolve_call_fn!(T0, T1, T2, T3, T4);
impl_resolve_call_fn!(T0, T1, T2, T3, T4, T5);
impl_resolve_call_fn!(T0, T1, T2, T3, T4, T5, T6);
impl_resolve_call_fn!(T0, T1, T2, T3, T4, T5, T6, T6);
impl_resolve_call_fn!(T0, T1, T2, T3, T4, T5, T6, T6, T8);
impl_resolve_call_fn!(T0, T1, T2, T3, T4, T5, T6, T6, T8, T9);
impl_resolve_call_fn!(T0, T1, T2, T3, T4, T5, T6, T6, T8, T9, T10);
impl_resolve_call_fn!(T0, T1, T2, T3, T4, T5, T6, T6, T8, T9, T10, T11);

impl_resolve_call_fn!(mut T0);
impl_resolve_call_fn!(mut T0, T1);
impl_resolve_call_fn!(mut T0, T1, T2);
impl_resolve_call_fn!(mut T0, T1, T2, T3);
impl_resolve_call_fn!(mut T0, T1, T2, T3, T4);
impl_resolve_call_fn!(mut T0, T1, T2, T3, T4, T5);
impl_resolve_call_fn!(mut T0, T1, T2, T3, T4, T5, T6);
impl_resolve_call_fn!(mut T0, T1, T2, T3, T4, T5, T6, T6);
impl_resolve_call_fn!(mut T0, T1, T2, T3, T4, T5, T6, T6, T8);
impl_resolve_call_fn!(mut T0, T1, T2, T3, T4, T5, T6, T6, T8, T9);
impl_resolve_call_fn!(mut T0, T1, T2, T3, T4, T5, T6, T6, T8, T9, T10);
impl_resolve_call_fn!(mut T0, T1, T2, T3, T4, T5, T6, T6, T8, T9, T10, T11);

#[cfg(test)]
mod tests {
    use crate::resolve_call::{ResolveCall, ResolveCallMut};
    use dilib::Container;
    use std::sync::Mutex;

    #[test]
    fn resolve_call_test_1() {
        let repeater = |a: &String| {
            return a.repeat(2);
        };

        let mut container = Container::new();
        container.add_scoped(|| String::from("hello")).unwrap();

        let result = repeater.resolve_call(&container);
        assert_eq!(result, "hellohello");
    }

    #[test]
    fn resolve_call_fn_test_2() {
        let repeater = move |a: &String, count: &Mutex<usize>| {
            let count = *count.lock().expect("unable to get lock");
            return a.repeat(count);
        };

        let mut container = Container::new();
        container.add_scoped(|| String::from("adios!")).unwrap();
        container.add_singleton(Mutex::new(3_usize)).unwrap();

        let result = repeater.resolve_call(&container);
        assert_eq!(result, "adios!adios!adios!".to_owned());
    }

    #[test]
    fn resolve_call_mut_test_1() {
        let mut repeater = |a: &String| {
            return a.repeat(2);
        };

        let mut container = Container::new();
        container.add_scoped(|| String::from("apple")).unwrap();

        let result = repeater.resolve_call_mut(&container);
        assert_eq!(result, "appleapple");
    }

    #[test]
    fn resolve_call_fn_mut_test_2() {
        let mut repeater = move |a: &String, count: &Mutex<usize>| {
            let count = *count.lock().expect("unable to get lock");
            return a.repeat(count);
        };

        let mut container = Container::new();
        container.add_scoped(|| String::from("orange!")).unwrap();
        container.add_singleton(Mutex::new(4_usize)).unwrap();

        let result = repeater.resolve_call_mut(&container);
        assert_eq!(result, "orange!orange!orange!orange!".to_owned());
    }
}
