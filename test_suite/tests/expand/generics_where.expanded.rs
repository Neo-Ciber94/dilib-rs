use dilib::{Singleton, Injectable};
struct StructWithWhere<A, B, C>
where
    A: Clone,
    B: Default,
    C: Copy,
{
    a: A,
    b: Singleton<B>,
    c: Box<C>,
}
