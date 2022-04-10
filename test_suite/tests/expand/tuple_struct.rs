use dilib::{Singleton, Inject};

#[derive(Inject)]
struct TupleStruct(Singleton<usize>, String, bool);