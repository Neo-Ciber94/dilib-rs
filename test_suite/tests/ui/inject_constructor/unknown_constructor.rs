use dilib::Injectable;

#[derive(Injectable)]
#[inject(constructor="new(a, b)")]
struct MyStruct {
    a: String,
    b: usize
}

fn main(){}