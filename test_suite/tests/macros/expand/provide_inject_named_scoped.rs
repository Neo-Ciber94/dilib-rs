use dilib::{Inject, provide};

#[derive(Inject)]
#[provide(name = "point")]
struct MyData {
    #[inject(name="x")]
    x: i32,
    #[inject(name="y")]
    y: i32,
}