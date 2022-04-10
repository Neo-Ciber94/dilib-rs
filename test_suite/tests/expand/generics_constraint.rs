use dilib::{Singleton, Inject};

#[derive(Inject)]
struct StructWithConstraints<A: Clone, B: Default, C: Copy> {
    a: A,
    b: Singleton<B>,
    c: Box<C>
}