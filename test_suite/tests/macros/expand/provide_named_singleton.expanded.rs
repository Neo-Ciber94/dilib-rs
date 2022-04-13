use dilib::provide;
use std::sync::Mutex;
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[ctor::ctor]
    fn dilib_get_next_id_Mutex_usize() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::with_name::<Mutex<usize>>("id"),
            provider: dilib::Provider::Singleton(dilib::Shared::new(get_next_id())),
        });
    }
};
fn get_next_id() -> Mutex<usize> {
    Mutex::new(0)
}
