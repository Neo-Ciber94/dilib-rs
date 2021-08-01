use dilib::Injectable;
struct UnitStruct;
impl dilib::Injectable for UnitStruct {
    fn resolve(_: &dilib::Container) -> Self {
        UnitStruct
    }
}
