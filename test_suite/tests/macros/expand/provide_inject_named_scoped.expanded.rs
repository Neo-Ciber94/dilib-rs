use dilib::{Inject, provide};
#[inject(named = "point")]
struct MyData {
    x: i32,
    y: i32,
}
