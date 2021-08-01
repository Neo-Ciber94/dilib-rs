use dilib::{Singleton, Injectable};
struct DefaultTyParam<A = usize, B = String, C = bool> {
    a: A,
    b: Singleton<B>,
    c: Box<C>,
}
impl<A, B, C> dilib::Injectable for DefaultTyParam<A, B, C> {
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
        DefaultTyParam { a, b, c }
    }
}
