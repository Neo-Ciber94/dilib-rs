use dilib::Inject;
struct MyStruct {
    scoped: std::vec::Vec<String>,
}
impl dilib::Inject for MyStruct {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let scoped: std::vec::Vec<String> = container
            .get_scoped()
            .expect("cannot get scoped value of type `std::vec::Vec<String>`");
        MyStruct { scoped }
    }
}
