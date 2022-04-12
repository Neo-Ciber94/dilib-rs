use dilib::provide;
use std::sync::Mutex;
#[cold]
#[doc(hidden)]
#[ctor::ctor]
fn get_next_id__f0b05cf9d29d484dbfc34a8f541fe1bf() {
    let mut lock = dilib::global::PROVIDERS
        .lock()
        .expect("unable to get providers lock");
    let providers = lock.as_mut().expect("unable to get providers");
    providers.push(dilib::global::InjectProvider {
        key: dilib::InjectionKey::with_name::<Mutex<usize>>("id"),
        provider: dilib::Provider::Singleton(dilib::Shared::new(get_next_id())),
    });
}
#[allow(dead_code)]
fn get_next_id() -> Mutex<usize> {
    Mutex::new(0)
}
