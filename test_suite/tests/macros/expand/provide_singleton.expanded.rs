use dilib::provide;
use std::sync::Mutex;
#[cold]
#[doc(hidden)]
#[ctor::ctor]
fn get_counter__b1a37c2ae4014ff5b4bd4ce886e2e250() {
    let mut lock = dilib::global::PROVIDERS
        .lock()
        .expect("unable to get providers lock");
    let providers = lock.as_mut().expect("unable to get providers");
    providers.push(dilib::global::InjectProvider {
        key: dilib::InjectionKey::of::<Mutex<u32>>(),
        provider: dilib::Provider::Singleton(dilib::Shared::new(get_counter())),
    });
}
#[allow(dead_code)]
fn get_counter() -> Mutex<u32> {
    Mutex::new(10)
}
