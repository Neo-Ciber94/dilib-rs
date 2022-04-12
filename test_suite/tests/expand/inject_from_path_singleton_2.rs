use dilib::Inject;

#[derive(Inject)]
struct MyStruct {
    singleton: std::sync::Arc<std::option::Option<String>>
}