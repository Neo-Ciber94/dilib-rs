use dilib::provide;
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" fn dilib_get_scoped_trait_usize() {
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
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".CRT$XCU"]
    static dilib_get_scoped_trait_usize___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn dilib_get_scoped_trait_usize___rust_ctor___ctor() {
            dilib_get_scoped_trait_usize()
        };
        dilib_get_scoped_trait_usize___rust_ctor___ctor
    };
};
fn get_scoped_trait() -> usize {
    123
}
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" fn dilib_get_multiple_scoped_trait_usize() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::of::<std::boxed::Box<dyn std::ops::Mul<usize> + Send + Sync>>(
            ),
            provider: dilib::Provider::Scoped(dilib::Scoped::from_factory(|| {
                std::boxed::Box::<std::boxed::Box<dyn std::ops::Mul<usize> + Send + Sync>>::new(
                    get_multiple_scoped_trait(),
                )
            })),
        });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".CRT$XCU"]
    static dilib_get_multiple_scoped_trait_usize___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn dilib_get_multiple_scoped_trait_usize___rust_ctor___ctor() {
            dilib_get_multiple_scoped_trait_usize()
        };
        dilib_get_multiple_scoped_trait_usize___rust_ctor___ctor
    };
};
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" fn dilib_get_multiple_scoped_trait_usize() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::of::<std::boxed::Box<dyn std::ops::Add<usize> + Send + Sync>>(
            ),
            provider: dilib::Provider::Scoped(dilib::Scoped::from_factory(|| {
                std::boxed::Box::<std::boxed::Box<dyn std::ops::Add<usize> + Send + Sync>>::new(
                    get_multiple_scoped_trait(),
                )
            })),
        });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".CRT$XCU"]
    static dilib_get_multiple_scoped_trait_usize___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn dilib_get_multiple_scoped_trait_usize___rust_ctor___ctor() {
            dilib_get_multiple_scoped_trait_usize()
        };
        dilib_get_multiple_scoped_trait_usize___rust_ctor___ctor
    };
};
fn get_multiple_scoped_trait() -> usize {
    123
}
