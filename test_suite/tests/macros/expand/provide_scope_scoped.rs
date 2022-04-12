use dilib::provide;

#[provide(scope="scoped")]
fn get_greet() -> String {
    "Hello, world!".to_string()
}