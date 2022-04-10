use dilib::{Singleton, Inject};

#[derive(Inject)]
struct InjectStruct {
    a: Singleton<usize>,
    b: String,
    c: Box<bool>
}