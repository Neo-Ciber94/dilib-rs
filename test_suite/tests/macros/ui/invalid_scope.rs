use dilib::provide;

#[provide(scope="global")]
fn get_string() -> String {
    "hello".to_string()
}

fn main(){}