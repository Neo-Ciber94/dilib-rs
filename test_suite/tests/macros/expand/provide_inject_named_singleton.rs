use dilib::{Inject, Singleton, provide};

#[derive(Inject)]
#[provide(scope="singleton", name = "point")]
struct MyData {
    s: Singleton<String>
}