use dilib::{Singleton, Injectable};
use std::sync::{Arc, Mutex};
type UsizeSingleton = Arc<Mutex<usize>>;
struct StructInjectScope {
    #[inject(scope = "singleton")]
    a: UsizeSingleton,
    #[inject(scope = "scoped")]
    b: Box<i32>,
}
