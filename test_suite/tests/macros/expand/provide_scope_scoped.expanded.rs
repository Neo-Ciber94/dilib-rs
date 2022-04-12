use dilib::provide;
#[cold]
#[doc(hidden)]
#[ctor::ctor]
fn get_greet__2cc3381fce7b41c3aba164cf58971a12() {
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
