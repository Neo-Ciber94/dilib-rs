use dilib::{Singleton, Injectable};
struct StructInjectDefault {
    #[inject(default = "Lorem ipsum")]
    a: String,
    #[inject(default = 10)]
    b: usize,
    #[inject(default)]
    c: bool,
}
impl dilib::Injectable for StructInjectDefault {
    fn resolve(container: &dilib::Container) -> Self {
        let a: String = "Lorem ipsum".into();
        let b: usize = 10;
        let c: bool = std::default::Default::default();
        StructInjectDefault { a, b, c }
    }
}
