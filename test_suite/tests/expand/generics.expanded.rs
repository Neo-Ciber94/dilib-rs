use dilib::{Singleton, Injectable};
struct StructWithGenerics<A, B, C> {
    a: A,
    b: Singleton<B>,
    c: Box<C>,
}
