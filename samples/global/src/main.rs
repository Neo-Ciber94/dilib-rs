use dilib::global::{init_container};
use dilib::{
    get_scoped, get_singleton, Container, Injectable,
    proc_macros::{provide, inject}
};
use std::sync::{Arc, Mutex};

#[provide]
fn hello() -> String {
    "Hello".to_string()
}

#[provide(scope="singleton", name="count", order=2)]
#[cold] // You may require to use #[cold] because the function will only be called once
fn counter() -> Mutex<usize> {
    Mutex::new(0)
}

#[provide(order=1000)]
struct PrintCount {
    counter: Arc<Mutex<usize>>,
    greet: String,
}

impl PrintCount {
    fn print(&self) {
        let counter = self.counter.lock().expect("unable to get counter lock");
        let val = *counter;
        println!("{} {}", self.greet, val);
    }
}

impl Injectable for PrintCount {
    fn resolve(container: &Container) -> Self {
        let counter = container
            .get_singleton_with_name::<Mutex<usize>>("count")
            .expect("unable to get counter");

        let greet = container
            .get_scoped::<String>()
            .expect("unable to get greet");

        PrintCount { counter, greet }
    }
}


#[provide(name="hola")]
#[inject(counter, name="count")]
fn say_hola(counter: Arc<Mutex<usize>>) -> String {
    let lock = counter.lock().expect("unable to get counter lock");
    let val = *lock;
    format!("Hola {}", val)
}

fn main() {
    init_container(|_| {})
    .unwrap();

    let hello = get_scoped!(String).expect("unable to get hello");
    let count = get_singleton!(Mutex<usize>, "count").expect("unable to get count");

    {
        let mut lock = count.lock().expect("unable to get counter lock");
        *lock += 5;
    }

    let hola = get_scoped!(String, "hola").expect("unable to get hola");
    let print_count = get_scoped!(PrintCount).expect("unable to get print count");

    println!("{}", hello);
    print_count.print();
    println!("{}", hola);
}
