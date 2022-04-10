use dilib::{Singleton, Inject};
use std::sync::{Arc, Mutex};
struct StructSingleton {
    a: Singleton<usize>,
    b: Arc<Mutex<usize>>,
}
impl dilib::Inject for StructSingleton {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let a: dilib::Singleton<usize> = container
            .get_singleton()
            .expect("cannot get singleton value of type `usize`");
        let b: dilib::Singleton<Mutex<usize>> = container
            .get_singleton()
            .expect("cannot get singleton value of type `Mutex<usize>`");
        StructSingleton { a, b }
    }
}
