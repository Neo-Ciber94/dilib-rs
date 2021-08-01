use dilib::Injectable;

#[derive(Injectable)]
#[inject(constructor = "new(a, b)")]
struct MyStruct {
    a: String,
    b: i32
}

impl MyStruct {
    pub fn new(b: i32, a: String) -> Self {
        MyStruct { a, b }
    }
}

fn main(){}