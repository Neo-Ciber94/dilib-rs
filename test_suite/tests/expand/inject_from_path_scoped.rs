use dilib::Inject;

#[derive(Inject)]
struct MyStruct {
    scoped: std::vec::Vec<String>
}