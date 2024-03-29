use dilib::provide;
use std::fmt::Display;
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" fn dilib_get_singleton_trait_String() {
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
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".CRT$XCU"]
    static dilib_get_singleton_trait_String___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn dilib_get_singleton_trait_String___rust_ctor___ctor() {
            dilib_get_singleton_trait_String()
        };
        dilib_get_singleton_trait_String___rust_ctor___ctor
    };
};
fn get_singleton_trait() -> String {
    "hola".to_string()
}
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" fn dilib_get_multiple_singleton_trait_String() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::of::<std::boxed::Box<dyn Debug + Send + Sync>>(),
            provider: dilib::Provider::Scoped(dilib::Scoped::from_factory(|| {
                std::boxed::Box::<std::boxed::Box<dyn Debug + Send + Sync>>::new(
                    get_multiple_singleton_trait(),
                )
            })),
        });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".CRT$XCU"]
    static dilib_get_multiple_singleton_trait_String___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn dilib_get_multiple_singleton_trait_String___rust_ctor___ctor() {
            dilib_get_multiple_singleton_trait_String()
        };
        dilib_get_multiple_singleton_trait_String___rust_ctor___ctor
    };
};
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" fn dilib_get_multiple_singleton_trait_String() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::of::<std::boxed::Box<dyn Display + Send + Sync>>(),
            provider: dilib::Provider::Scoped(dilib::Scoped::from_factory(|| {
                std::boxed::Box::<std::boxed::Box<dyn Display + Send + Sync>>::new(
                    get_multiple_singleton_trait(),
                )
            })),
        });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".CRT$XCU"]
    static dilib_get_multiple_singleton_trait_String___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn dilib_get_multiple_singleton_trait_String___rust_ctor___ctor() {
            dilib_get_multiple_singleton_trait_String()
        };
        dilib_get_multiple_singleton_trait_String___rust_ctor___ctor
    };
};
fn get_multiple_singleton_trait() -> String {
    "hola".to_string()
}
