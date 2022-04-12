use dilib::{Inject, provide};

#[derive(Inject)]
#[inject(named = "point")]
struct MyData {
    x: i32,
    y: i32,
}