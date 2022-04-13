use dilib::provide;
use std::sync::{Arc, Mutex};
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[ctor::ctor]
    fn dilib_compute_singleton_Mutex_String() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::of::<Mutex<String>>(),
            provider: dilib::Provider::Singleton(dilib::Shared::from_factory(
                |container: &dilib::Container| -> Mutex<String> {
                    let x = container
                        .get_scoped::<i32>()
                        .expect("failed to resolve: i32");
                    let y = container
                        .get_singleton::<u32>()
                        .expect("failed to resolve: u32");
                    compute_singleton(x, y)
                },
            )),
        });
    }
};
fn compute_singleton(x: i32, y: Arc<u32>) -> Mutex<String> {
    let z = x + y;
    Mutex::new({
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &[""],
            &[::core::fmt::ArgumentV1::new_display(&z)],
        ));
        res
    })
}
