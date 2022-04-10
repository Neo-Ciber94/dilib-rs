use dilib::Inject;

#[derive(Inject)]
#[inject(constructor="new(a, b)")]
struct MyStruct {
    a: String,
    b: usize
}

fn main(){}