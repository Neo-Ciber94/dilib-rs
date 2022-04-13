use dilib::provide;
use std::sync::Arc;

#[provide(scope="scoped", name="scoped_data")]
fn compute_named_scoped(x: i32, y: Arc<u32>) -> String {
    let z = x + y;
    format!("{}", z)
}