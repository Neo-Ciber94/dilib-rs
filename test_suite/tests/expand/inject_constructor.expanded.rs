use dilib::{Singleton, Inject};
#[inject(constructor = "new(a, b, c)")]
struct StructInjectConstructor {
    a: String,
    b: Singleton<usize>,
    c: Box<bool>,
}
impl dilib::Inject for StructInjectConstructor {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let a: String = container
            .get_scoped()
            .expect("cannot get scoped value of type `String`");
        let b: dilib::Singleton<usize> = container
            .get_singleton()
            .expect("cannot get singleton value of type `usize`");
        let c: Box<bool> = container
            .get_scoped()
            .expect("cannot get scoped value of type `Box<bool>`");
        StructInjectConstructor::new(a, b, c)
    }
}
impl StructInjectConstructor {
    pub fn new(x: String, y: Singleton<usize>, z: Box<bool>) -> Self {
        StructInjectConstructor { a: x, b: y, c: z }
    }
}
