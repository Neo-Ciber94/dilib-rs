use dilib::{Singleton, Injectable};

#[derive(Injectable)]
struct TupleStruct(Singleton<usize>, String, bool);