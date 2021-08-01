use dilib::{Singleton, Injectable};

#[derive(Injectable)]
struct InjectStruct {
    a: Singleton<usize>,
    b: String,
    c: bool
}