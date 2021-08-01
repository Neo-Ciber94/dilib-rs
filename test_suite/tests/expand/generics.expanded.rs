use dilib::{Singleton, Injectable};
struct StructWithGenerics<A, B, C> {
    a: A,
    b: Singleton<B>,
    c: Box<C>,
}
impl<A, B, C> dilib::Injectable for StructWithGenerics<A, B, C> {
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
        StructWithGenerics { a, b, c }
    }
}
