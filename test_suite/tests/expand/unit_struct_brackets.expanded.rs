use dilib::Inject;
struct UnitStructBrackets {}
impl dilib::Inject for UnitStructBrackets {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        UnitStructBrackets {}
    }
}
