use dilib::provide;
use std::sync::Arc;
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" fn dilib_compute_scoped_String() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::of::<String>(),
            provider: dilib::Provider::Scoped(dilib::Scoped::from_construct(
                |container: &dilib::Container| -> String {
                    let x = container
                        .get_scoped::<i32>()
                        .expect("failed to resolve: i32");
                    let y = container
                        .get_singleton::<u32>()
                        .expect("failed to resolve: u32");
                    compute_scoped(x, y)
                },
            )),
        });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".CRT$XCU"]
    static dilib_compute_scoped_String___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn dilib_compute_scoped_String___rust_ctor___ctor() {
            dilib_compute_scoped_String()
        };
        dilib_compute_scoped_String___rust_ctor___ctor
    };
};
fn compute_scoped(x: i32, y: Arc<u32>) -> String {
    let z = x + y;
    {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &[""],
            &[::core::fmt::ArgumentV1::new_display(&z)],
        ));
        res
    }
}
