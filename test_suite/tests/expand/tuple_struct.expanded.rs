use dilib::{Singleton, Injectable};
struct TupleStruct(Singleton<usize>, String, bool);
impl dilib::Injectable for TupleStruct {
    fn resolve(container: &dilib::Container) -> Self {
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
