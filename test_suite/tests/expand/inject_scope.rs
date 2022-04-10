use dilib::{Singleton, Inject};
use std::sync::{Arc, Mutex};

type UsizeSingleton = Arc<Mutex<usize>>;

#[derive(Inject)]
struct StructInjectScope {
    #[inject(scope="singleton")]
    a: UsizeSingleton,

    // This actually does nothing because the default is scoped
    #[inject(scope="scoped")]
    b: Box<i32>
}