use dilib::provide;
use std::sync::Mutex;
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[ctor::ctor]
    fn dilib_get_counter_Mutex_u32() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::of::<Mutex<u32>>(),
            provider: dilib::Provider::Singleton(dilib::Shared::new(get_counter())),
        });
    }
};
fn get_counter() -> Mutex<u32> {
    Mutex::new(10)
}
