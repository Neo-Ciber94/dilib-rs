use dilib::provide;
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[ctor::ctor]
    fn dilib_get_greet_String() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::with_name::<String>("es_greet"),
            provider: dilib::Provider::Scoped(dilib::Scoped::from_factory(get_greet)),
        });
    }
};
fn get_greet() -> String {
    "Hola Mundo!".to_string()
}
