use dilib::Injectable;

#[derive(Injectable)]
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