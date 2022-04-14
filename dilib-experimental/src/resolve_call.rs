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
                    $(container.get::<$t>().as_ref().unwrap()),+
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