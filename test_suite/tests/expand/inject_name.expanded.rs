use dilib::{Singleton, Inject};
struct StructInjectName {
    #[inject(name = "API_KEY")]
    a: String,
    #[inject(name = "count")]
    b: Singleton<usize>,
    #[inject(name = "named")]
    c: Box<bool>,
}
impl dilib::Inject for StructInjectName {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let a: String = container
            .get_scoped_with_name("API_KEY")
            .expect("cannot get scoped value of name \"API_KEY\"");
        let b: dilib::Singleton<usize> = container
            .get_singleton_with_name("count")
            .expect("cannot get singleton value of name \"count\"");
        let c: Box<bool> = container
            .get_scoped_with_name("named")
            .expect("cannot get scoped value of name \"named\"");
        StructInjectName { a, b, c }
    }
}
