use dilib::{provide, Inject};

#[provide]
#[derive(Inject)]
struct MyStruct {
    field: u32,
}