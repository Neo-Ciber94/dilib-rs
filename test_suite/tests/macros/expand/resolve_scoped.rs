use dilib::provide;
use std::sync::Arc;

#[provide(scope="scoped")]
fn compute_scoped(x: i32, y: Arc<u32>) -> String {
    let z = x + y;
    format!("{}", z)
}