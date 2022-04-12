use dilib::provide;

#[provide(scope="invalid_scope")]
fn get_string() -> String {
    "hello".to_string()
}

fn main(){}