use dilib::{Singleton, Inject};
struct TupleStruct(Singleton<usize>, String, bool);
impl dilib::Inject for TupleStruct {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let _0: dilib::Singleton<usize> = container
            .get_singleton()
            .expect("cannot get singleton value of type `usize`");
        let _1: String = container
            .get_scoped()
            .expect("cannot get scoped value of type `String`");
        let _2: bool = container
            .get_scoped()
            .expect("cannot get scoped value of type `bool`");
        TupleStruct(_0, _1, _2)
    }
}
