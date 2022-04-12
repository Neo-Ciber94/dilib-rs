use dilib::provide;
#[cold]
#[doc(hidden)]
#[ctor::ctor]
fn get_greet__ee230e7baa2b4bb18924b4fed3963e91() {
    let mut lock = dilib::global::PROVIDERS
        .lock()
        .expect("unable to get providers lock");
    let providers = lock.as_mut().expect("unable to get providers");
    providers.push(dilib::global::InjectProvider {
        key: dilib::InjectionKey::with_name::<String>("es_greet"),
        provider: dilib::Provider::Scoped(dilib::Scoped::from_factory(get_greet)),
    });
}
#[allow(dead_code)]
fn get_greet() -> String {
    "Hola Mundo!".to_string()
}
