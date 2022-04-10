use dilib::{Singleton, Inject};

#[derive(Inject)]
struct StructWithGenerics<A, B, C> {
    a: A,
    b: Singleton<B>,
    c: Box<C>
}