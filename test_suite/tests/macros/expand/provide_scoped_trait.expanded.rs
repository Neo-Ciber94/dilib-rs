use dilib::provide;
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[ctor::ctor]
    fn dilib_get_scoped_trait_usize() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::of::<std::boxed::Box<dyn std::ops::Add<usize> + Send + Sync>>(
            ),
            provider: dilib::Provider::Scoped(dilib::Scoped::from_factory(|| {
                std::boxed::Box::<std::boxed::Box<dyn std::ops::Add<usize> + Send + Sync>>::new(
                    get_scoped_trait(),
                )
            })),
        });
    }
};
fn get_scoped_trait() -> usize {
    123
}
