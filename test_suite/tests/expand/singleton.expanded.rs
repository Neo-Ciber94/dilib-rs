use dilib::{Singleton, Injectable};
use std::sync::{Arc, Mutex};
struct StructSingleton {
    a: Singleton<usize>,
    b: Arc<Mutex<usize>>,
}
