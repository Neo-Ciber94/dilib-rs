use dilib::provide;
use std::fmt::Display;
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[ctor::ctor]
    fn dilib_get_singleton_trait_String() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::of::<std::boxed::Box<dyn Display + Send + Sync>>(),
            provider: dilib::Provider::Scoped(dilib::Scoped::from_factory(|| {
                std::boxed::Box::<std::boxed::Box<dyn Display + Send + Sync>>::new(
                    get_singleton_trait(),
                )
            })),
        });
    }
};
fn get_singleton_trait() -> String {
    "hola".to_string()
}
