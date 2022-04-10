use dilib::{
    get_scoped, get_singleton, global::init_container, macros::provide, Injectable, Singleton,
};
use std::sync::Mutex;
use dilib::global::get_container;

#[provide]
fn message() -> String {
    "Current count".to_string()
}

#[provide(scope = "singleton", name = "count")]
fn counter() -> Mutex<usize> {
    Mutex::new(0)
}

#[provide]
#[derive(Injectable)]
struct PrintCount {
    #[inject(name = "count")]
    counter: Singleton<Mutex<usize>>,
    msg: String,
}

impl PrintCount {
    fn print(&self) -> String {
        let count = self.counter.lock().unwrap();
        format!("{} {}", self.msg, count)
    }
}

fn main() {
    // Required to register the providers
    init_container(|_| {}).unwrap();

    println!("{}", get_container().unwrap().len());

    let counter = get_singleton!(Mutex<usize>, "count").expect("Could not get counter");
    *counter.lock().unwrap() += 5;

    let print_count = get_scoped!(PrintCount).expect("Could not get print count");
    assert_eq!(print_count.print(), "Current count 5");

    println!("{}", print_count.print());
}
