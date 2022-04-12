use dilib::{Inject, provide};

#[derive(Inject)]
#[provide(scope="singleton")]
struct MyStruct {
    x: usize,
    y: Box<String>
}