use dilib::Inject;

#[derive(Inject)]
struct MyStruct {
    singleton: dilib::Singleton<std::option::Option<String>>
}