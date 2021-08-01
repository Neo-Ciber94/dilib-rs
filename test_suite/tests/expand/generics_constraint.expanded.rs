use dilib::{Singleton, Injectable};
struct StructWithConstraints<A: Clone, B: Default, C: Copy> {
    a: A,
    b: Singleton<B>,
    c: Box<C>,
}
impl<A: Clone, B: Default, C: Copy> dilib::Injectable for StructWithConstraints<A, B, C> {
    fn resolve(container: &dilib::Container) -> Self {
        let a: A = container
            .get_scoped()
            .expect("cannot get scoped value of type `A`");
        let b: dilib::Singleton<B> = container
            .get_singleton()
            .expect("cannot get singleton value of type `B`");
        let c: Box<C> = container
            .get_scoped()
            .expect("cannot get scoped value of type `Box<C>`");
        StructWithConstraints { a, b, c }
    }
}
