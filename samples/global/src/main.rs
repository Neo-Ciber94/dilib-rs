use dilib::{Inject, Singleton};

#[derive(Inject)]
struct InjectStruct {
    a: Singleton<usize>,
    b: String,
    c: Box<bool>,
}

fn main() {}
