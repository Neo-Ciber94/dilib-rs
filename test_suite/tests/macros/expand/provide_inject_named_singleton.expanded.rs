use dilib::{Inject, Singleton, provide};
#[inject(scope = "singleton", named = "point")]
struct MyData {
    s: Singleton<String>,
}
