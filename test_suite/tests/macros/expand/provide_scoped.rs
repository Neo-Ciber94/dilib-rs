use dilib::provide;

#[provide]
fn get_greet() -> String {
    "Hello, world!".to_string()
}