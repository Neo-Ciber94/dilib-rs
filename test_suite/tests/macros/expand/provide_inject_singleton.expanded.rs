use dilib::{Inject, provide};
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" fn dilib_MyStruct_MyStruct() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::of::<MyStruct>(),
            provider: dilib::Provider::Singleton(dilib::Shared::new_lazy(
                |container: &dilib::Container| -> MyStruct {
                    <MyStruct as dilib::Inject>::inject(container)
                },
            )),
        });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".CRT$XCU"]
    static dilib_MyStruct_MyStruct___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn dilib_MyStruct_MyStruct___rust_ctor___ctor() {
            dilib_MyStruct_MyStruct()
        };
        dilib_MyStruct_MyStruct___rust_ctor___ctor
    };
};
struct MyStruct {
    val: String,
}
impl dilib::Inject for MyStruct {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let val: String = container
            .get_scoped()
            .expect("cannot get scoped value of type `String`");
        MyStruct { val }
    }
}
