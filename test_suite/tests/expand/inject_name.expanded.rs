use dilib::{Singleton, Injectable};
struct StructInjectName {
    #[inject(name = "API_KEY")]
    a: String,
    #[inject(name = "count")]
    b: Singleton<usize>,
    #[inject(name = "named")]
    c: Box<bool>,
}
