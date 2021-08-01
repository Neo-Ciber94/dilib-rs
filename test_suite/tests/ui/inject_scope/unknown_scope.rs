use dilib::{Singleton, Injectable};

#[derive(Injectable)]
struct MyStruct {
    #[inject(scope="unique")]
    a: Singleton<usize>,
}

fn main(){}