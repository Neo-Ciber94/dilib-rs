use dilib::{Singleton, Injectable};
use std::sync::{Arc, Mutex};

type UsizeSingleton = Arc<Mutex<usize>>;

#[derive(Injectable)]
struct StructInjectScope {
    #[inject(scope="singleton")]
    a: UsizeSingleton,

    // This actually does nothing because the default is scoped
    #[inject(scope="scoped")]
    b: Box<i32>
}