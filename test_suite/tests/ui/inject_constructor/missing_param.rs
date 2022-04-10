use dilib::{Singleton, Inject};

#[derive(Inject)]
#[inject(constructor="new(a, b)")]
struct MyStruct {
    a: String,
    b: Singleton<usize>,
}

impl MyStruct {
    pub fn new(a: String, b: Singleton<usize>, trim_string: bool) -> Self {
        if trim_string {
            a = a.trim().to_owned();
        }

        MyStruct { a, b }
    }
}

fn main(){}