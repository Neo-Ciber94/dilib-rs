use dilib::{Singleton, Injectable};

#[derive(Injectable)]
struct StructWithGenerics<A, B, C> {
    a: A,
    b: Singleton<B>,
    c: Box<C>
}