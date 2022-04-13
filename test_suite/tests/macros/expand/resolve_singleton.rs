use dilib::provide;
use std::sync::{Arc, Mutex};

#[provide(scope="singleton")]
fn compute_singleton(x: i32, y: Arc<u32>) -> Mutex<String> {
    let z = x + y;
    Mutex::new(format!("{}", z))
}