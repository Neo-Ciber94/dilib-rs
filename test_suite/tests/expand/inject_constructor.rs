use dilib::{Singleton, Inject};

#[derive(Inject)]
#[inject(constructor="new(a, b, c)")]
struct StructInjectConstructor {
    a: String,
    b: Singleton<usize>,
    c: Box<bool>
}

impl StructInjectConstructor {
    pub fn new(x: String, y: Singleton<usize>, z: Box<bool>) -> Self {
        StructInjectConstructor { a: x, b: y, c: z}
    }
}