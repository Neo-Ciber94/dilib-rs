use dilib::{Singleton, Inject};

#[derive(Inject)]
struct MyStruct {
    #[inject(scope="unique")]
    a: Singleton<usize>,
}

fn main(){}