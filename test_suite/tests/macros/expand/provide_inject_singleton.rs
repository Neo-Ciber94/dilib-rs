use dilib::{Inject, provide};

#[derive(Inject)]
#[provide(scope="singleton")]
struct MyStruct {
    val: String
}