use dilib::{Singleton, Injectable};

#[derive(Injectable)]
struct StructWithLifeTimes<'a, 'b> {
    a: &'a String,
    b: Singleton<&'a usize>,
    c: &'static str,
}