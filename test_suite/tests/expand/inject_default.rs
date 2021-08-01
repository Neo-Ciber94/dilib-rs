use dilib::{Singleton, Injectable};

#[derive(Injectable)]
struct StructInjectDefault {
    #[inject(default="Lorem ipsum")]
    a: String,
    #[inject(default=10)]
    b: usize,
    #[inject(default)]
    c: bool,
}