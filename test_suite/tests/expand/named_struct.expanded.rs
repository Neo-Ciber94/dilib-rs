use dilib::{Singleton, Injectable};
struct InjectStruct {
    a: Singleton<usize>,
    b: String,
    c: Box<bool>,
}
