use std::sync::Mutex;
use once_cell::sync::Lazy;
use dilib::{get_scoped, InjectionKey, Provider, Scoped, Injectable};
use dilib::global::init_container;

static PROVIDERS : Lazy<Mutex<Option<Vec<(InjectionKey<'static>, Provider)>>>> = Lazy::new(|| Default::default());


// #[provide(with="default")]
// struct Hello {
//     name: String,
// }
//
// #[provide(scope="singleton", name="es", order=1000)]
// pub fn say_hola(locale: Localizer) -> String {
//     "Hola".to_string()
// }
//
// #[provide]
// pub fn get_hello() -> String {
//     String::from("Hello World")
// }

//#[provide]
#[derive(Injectable)]
struct Greeting {
    #[inject(default="Hola Mundo")]
    greeting: String,
}

#[ctor::ctor]
fn init() {
    let mut lock = PROVIDERS.lock().unwrap();
    let providers = lock.get_or_insert_with(|| Vec::new());
    let key = InjectionKey::of::<usize>();
    let provider = Provider::Scoped(
        Scoped::from_injectable::<usize, _>(|c| {
            let x = c.get_scoped::<String>().unwrap();
            println!("{:?}", x);
            x.len()
        })
    );
    providers.push((key, provider));
}

fn main() {
    init_container(|c| {
        c.add_scoped(|| String::from("Hola"));
        let providers = PROVIDERS.lock().unwrap().take().unwrap();
        for (key, provider) in providers {
            c.__add_provider(key, provider);
        }
    }).unwrap();

    let val = get_scoped!(usize).unwrap();
    println!("{}", val);
}
