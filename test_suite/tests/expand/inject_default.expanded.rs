use dilib::{Singleton, Inject};
struct StructInjectDefault {
    #[inject(default = "Lorem ipsum")]
    a: String,
    #[inject(default = 10)]
    b: usize,
    #[inject(default)]
    c: bool,
}
impl dilib::Inject for StructInjectDefault {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let a: String = "Lorem ipsum".into();
        let b: usize = 10.into();
        let c: bool = std::default::Default::default();
        StructInjectDefault { a, b, c }
    }
}
