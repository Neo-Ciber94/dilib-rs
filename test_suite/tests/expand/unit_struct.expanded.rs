use dilib::Inject;
struct UnitStruct;
impl dilib::Inject for UnitStruct {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(_: &dilib::Container) -> Self {
        UnitStruct
    }
}
