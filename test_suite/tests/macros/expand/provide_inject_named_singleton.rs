use dilib::{Inject, Singleton, provide};

#[derive(Inject)]
#[inject(scope="singleton", named = "point")]
struct MyData {
    s: Singleton<String>
}