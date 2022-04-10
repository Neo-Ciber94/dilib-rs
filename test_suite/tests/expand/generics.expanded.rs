use dilib::{Singleton, Inject};
struct StructWithGenerics<A, B, C> {
    a: A,
    b: Singleton<B>,
    c: Box<C>,
}
impl<A, B, C> dilib::Inject for StructWithGenerics<A, B, C> {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
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
