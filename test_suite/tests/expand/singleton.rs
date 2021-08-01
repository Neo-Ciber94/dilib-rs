use dilib::{Singleton, Injectable};
use std::sync::{Arc, Mutex};

#[derive(Injectable)]
struct StructSingleton {
    a: Singleton<usize>,
    b: Arc<Mutex<usize>>
}