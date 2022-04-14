use dilib::provide;

trait MySingletonTrait {}

#[provide(scope="singleton", bind="MySingletonTrait")]
struct MyStruct {
    field: i32,
}

impl MySingletonTrait for MyStruct {}
