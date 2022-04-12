use dilib::provide;
use std::sync::Mutex;

#[provide(scope="singleton", name="id")]
fn get_next_id() -> Mutex<usize> {
    Mutex::new(0)
}