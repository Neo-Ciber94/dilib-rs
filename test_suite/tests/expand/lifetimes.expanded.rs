use dilib::{Singleton, Inject};
struct StructWithLifeTimes<'a, 'b> {
    a: &'a String,
    b: Singleton<&'a usize>,
    c: &'static str,
}
impl<'a, 'b> dilib::Inject for StructWithLifeTimes<'a, 'b> {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let a: &'a String = container
            .get_scoped()
            .expect("cannot get scoped value of type `&String`");
        let b: Singleton<&'a usize> = container
            .get_scoped()
            .expect("cannot get scoped value of type `Singleton<&usize>`");
        let c: &'static str = container
            .get_scoped()
            .expect("cannot get scoped value of type `&\'static str`");
        StructWithLifeTimes { a, b, c }
    }
}
