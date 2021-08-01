use dilib::{Singleton, Injectable};
struct InjectStruct {
    a: Singleton<usize>,
    b: String,
    c: Box<bool>,
}
impl dilib::Injectable for InjectStruct {
    fn resolve(container: &dilib::Container) -> Self {
        let a: dilib::Singleton<usize> = container
            .get_singleton()
            .expect("cannot get singleton value of type `usize`");
        let b: String = container
            .get_scoped()
            .expect("cannot get scoped value of type `String`");
        let c: Box<bool> = container
            .get_scoped()
            .expect("cannot get scoped value of type `Box<bool>`");
        InjectStruct { a, b, c }
    }
}
