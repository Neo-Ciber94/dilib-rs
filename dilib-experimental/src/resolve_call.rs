use dilib::Container;

pub trait ResolveCall<Args> {
    type Output;
    fn resolve_call(&self, container: &Container) -> Self::Output;
}

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
                            .unwrap_or_else(|| panic!("unable to get {}", stringify!($t)))
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
                    $(container.get::<$t>().as_ref().unwrap()),+
                )
            }
        }
    };
}

impl_resolve_call_fn!(A);
impl_resolve_call_fn!(A, B);
impl_resolve_call_fn!(A, B, C);
impl_resolve_call_fn!(A, B, C, D);
impl_resolve_call_fn!(A, B, C, D, E);
impl_resolve_call_fn!(A, B, C, D, E, F);
impl_resolve_call_fn!(A, B, C, D, E, F, G);
impl_resolve_call_fn!(A, B, C, D, E, F, G, H);
impl_resolve_call_fn!(A, B, C, D, E, F, G, H, I);
impl_resolve_call_fn!(A, B, C, D, E, F, G, H, I, J);
impl_resolve_call_fn!(A, B, C, D, E, F, G, H, I, J, K);
impl_resolve_call_fn!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_resolve_call_fn!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_resolve_call_fn!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_resolve_call_fn!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_resolve_call_fn!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);

impl_resolve_call_fn!(mut A);
impl_resolve_call_fn!(mut A, B);
impl_resolve_call_fn!(mut A, B, C);
impl_resolve_call_fn!(mut A, B, C, D);
impl_resolve_call_fn!(mut A, B, C, D, E);
impl_resolve_call_fn!(mut A, B, C, D, E, F);
impl_resolve_call_fn!(mut A, B, C, D, E, F, G);
impl_resolve_call_fn!(mut A, B, C, D, E, F, G, H);
impl_resolve_call_fn!(mut A, B, C, D, E, F, G, H, I);
impl_resolve_call_fn!(mut A, B, C, D, E, F, G, H, I, J);
impl_resolve_call_fn!(mut A, B, C, D, E, F, G, H, I, J, K);
impl_resolve_call_fn!(mut A, B, C, D, E, F, G, H, I, J, K, L);
impl_resolve_call_fn!(mut A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_resolve_call_fn!(mut A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_resolve_call_fn!(mut A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_resolve_call_fn!(mut A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use dilib::{Container, Singleton};
    use crate::resolve_call::ResolveCall;

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
        let repeater = |a: &String, count: &Singleton<Mutex<usize>>| {
            let count = *count.lock().expect("unable to get lock");
            return a.repeat(count);
        };

        let mut container = Container::new();
        container.add_scoped(|| String::from("adios!")).expect("unable to add scoped");
        container.add_singleton(Mutex::new(3)).expect("unable to add singleton");

        let result = repeater.resolve_call(&container);
        assert_eq!(result, "adios!adios!adios!".to_owned());
    }
}