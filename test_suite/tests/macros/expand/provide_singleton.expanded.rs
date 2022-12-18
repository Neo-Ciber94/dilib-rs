use dilib::provide;
use std::sync::Mutex;
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" fn dilib_get_counter_Mutex_u32() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::of::<Mutex<u32>>(),
            provider: dilib::Provider::Singleton(dilib::Shared::new(get_counter())),
        });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".CRT$XCU"]
    static dilib_get_counter_Mutex_u32___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn dilib_get_counter_Mutex_u32___rust_ctor___ctor() {
            dilib_get_counter_Mutex_u32()
        };
        dilib_get_counter_Mutex_u32___rust_ctor___ctor
    };
};
fn get_counter() -> Mutex<u32> {
    Mutex::new(10)
}
