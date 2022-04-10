use dilib::{Singleton, Inject};

#[derive(Inject)]
struct StructWithWhere<A, B, C> where A: Clone, B: Default, C: Copy {
    a: A,
    b: Singleton<B>,
    c: Box<C>
}