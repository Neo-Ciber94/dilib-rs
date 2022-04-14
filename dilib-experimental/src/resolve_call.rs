use dilib::Container;

pub trait ResolveCall {
    type Output;
    fn resolve_call(&self, container: &Container) -> Self::Output;
}

pub trait ResolveCallMut {
    type Output;
    fn resolve_call_mut(&mut self, container: &Container) -> Self::Output;
}

#[macro_export]
macro_rules! impl_resolve_call_fn {
    ($($t:ident),+) => {
        impl<Out, $($t),+> ResolveCall for Box<dyn Fn($(&$t),+) -> Out>
         where $($t: Send + Sync + 'static),+ {
            type Output = Out;
            fn resolve_call(&self, container: &Container) -> Self::Output {
                (self)(
                    $(container.get::<$t>().unwrap().as_ref()),+
                )
            }
        }
    };

    (mut $($t:ident),+) => {
        impl<Out, $($t),+> ResolveCallMut for Box<dyn FnMut($(&$t),+) -> Out>
         where $($t: Send + Sync + 'static),+ {
            type Output = Out;
            fn resolve_call_mut(&mut self, container: &Container) -> Self::Output {
                (self)(
                    $(container.get::<$t>().unwrap().as_ref()),+
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

// impl<Out, A> ResolveCall for Box<dyn Fn(&A) -> Out>
// where
//     A: Send + Sync + 'static,
// {
//     type Output = Out;
//
//     fn resolve_call(&self, container: &Container) -> Self::Output {
//         (self)(container.get::<A>().unwrap().as_ref())
//     }
// }
