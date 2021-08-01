use dilib::{Singleton, Injectable};
use std::sync::{Arc, Mutex};
type UsizeSingleton = Arc<Mutex<usize>>;
struct StructInjectScope {
    #[inject(scope = "singleton")]
    a: UsizeSingleton,
    #[inject(scope = "scoped")]
    b: Box<i32>,
}
impl dilib::Injectable for StructInjectScope {
    fn resolve(container: &dilib::Container) -> Self {
        let a: dilib::Singleton<UsizeSingleton> = container
            .get_singleton()
            .expect("cannot get singleton value of type `UsizeSingleton`");
        let b: Box<i32> = container
            .get_scoped()
            .expect("cannot get scoped value of type `Box<i32>`");
        StructInjectScope { a, b }
    }
}
