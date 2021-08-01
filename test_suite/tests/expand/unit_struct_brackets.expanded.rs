use dilib::Injectable;
struct UnitStructBrackets {}
impl dilib::Injectable for UnitStructBrackets {
    fn resolve(container: &dilib::Container) -> Self {
        UnitStructBrackets {}
    }
}
