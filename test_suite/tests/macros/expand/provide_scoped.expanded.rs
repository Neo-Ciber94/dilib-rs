use dilib::provide;
#[cold]
#[doc(hidden)]
#[ctor::ctor]
fn get_greet__29f4efb1f6a049bcb62104663b8f19c3() {
    let mut lock = dilib::global::PROVIDERS
        .lock()
        .expect("unable to get providers lock");
    let providers = lock.as_mut().expect("unable to get providers");
    providers.push(dilib::global::InjectProvider {
        key: dilib::InjectionKey::of::<String>(),
        provider: dilib::Provider::Scoped(dilib::Scoped::from_factory(get_greet)),
    });
}
#[allow(dead_code)]
fn get_greet() -> String {
    "Hello, world!".to_string()
}
