use dilib::{Singleton, Inject};
use std::sync::{Arc, Mutex};

#[derive(Inject)]
struct StructSingleton {
    a: Singleton<usize>,
    b: Arc<Mutex<usize>>
}