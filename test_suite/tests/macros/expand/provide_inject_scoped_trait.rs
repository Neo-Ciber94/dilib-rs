use dilib::provide;

trait MyScopedTrait {}

#[provide(scope="scoped", bind="MyScopedTrait")]
struct MyStruct {
    field: i32,
}

impl MyScopedTrait for MyStruct {}
