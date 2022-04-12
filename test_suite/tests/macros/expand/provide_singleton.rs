use dilib::provide;
use std::sync::Mutex;

#[provide(scope="singleton")]
fn get_counter() -> Mutex<u32> {
    Mutex::new(10)
}