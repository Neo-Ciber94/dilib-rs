use dilib::{Singleton, Inject};

#[derive(Inject)]
struct StructWithLifeTimes<'a, 'b> {
    a: &'a String,
    b: Singleton<&'a usize>,
    c: &'static str,
}