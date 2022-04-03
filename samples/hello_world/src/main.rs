use dilib::Container;

fn main() {
    let mut container = Container::new();
    container.add_singleton("Hello World".to_owned());
    container.add_scoped(|| "Hello Rust".to_owned());

    println!("{}", container.get_singleton::<String>().unwrap());
    println!("{}", container.get_scoped::<String>().unwrap());
}
