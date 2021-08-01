use dilib::{Singleton, Injectable};
use std::sync::{Arc, Mutex};
struct StructSingleton {
    a: Singleton<usize>,
    b: Arc<Mutex<usize>>,
}
impl dilib::Injectable for StructSingleton {
    fn resolve(container: &dilib::Container) -> Self {
        let a: dilib::Singleton<usize> = container
            .get_singleton()
            .expect("cannot get singleton value of type `usize`");
        let b: dilib::Singleton<Mutex<usize>> = container
            .get_singleton()
            .expect("cannot get singleton value of type `Mutex<usize>`");
        StructSingleton { a, b }
    }
}
