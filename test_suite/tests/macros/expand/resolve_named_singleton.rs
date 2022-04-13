use dilib::provide;
use std::sync::{Arc, Mutex};

#[provide(scope="singleton", name="bar")]
fn compute_named_singleton(x: i32, y: Arc<u32>) -> Mutex<String> {
    let z = x + y;
    Mutex::new(format!("{}", z))
}