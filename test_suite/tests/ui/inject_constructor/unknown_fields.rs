use dilib::Inject;

#[derive(Inject)]
#[inject(constructor="new(x, y)")]
struct MyStruct {
    a: String,
    b: i32
}

impl MyStruct {
    pub fn new(a: String, b: i32) -> Self {
        MyStruct { a, b }
    }
}

fn main(){}