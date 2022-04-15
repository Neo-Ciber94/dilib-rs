use dilib::provide;
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" fn dilib_get_greet_String() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::with_name::<String>("es_greet"),
            provider: dilib::Provider::Scoped(dilib::Scoped::from_factory(get_greet)),
        });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".CRT$XCU"]
    static dilib_get_greet_String___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn dilib_get_greet_String___rust_ctor___ctor() {
            dilib_get_greet_String()
        };
        dilib_get_greet_String___rust_ctor___ctor
    };
};
fn get_greet() -> String {
    "Hola Mundo!".to_string()
}
