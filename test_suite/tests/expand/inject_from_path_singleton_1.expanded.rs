use dilib::Inject;
struct MyStruct {
    singleton: dilib::Singleton<std::option::Option<String>>,
}
impl dilib::Inject for MyStruct {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let singleton: dilib::Singleton<std::option::Option<String>> = container
            .get_singleton()
            .expect("cannot get singleton value of type `std::option::Option<String>`");
        MyStruct { singleton }
    }
}
