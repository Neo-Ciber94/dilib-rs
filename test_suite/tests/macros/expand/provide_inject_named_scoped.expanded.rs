use dilib::{Inject, provide};
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" fn dilib_MyData_MyData() {
        let mut lock = dilib::global::PROVIDERS
            .lock()
            .expect("unable to get providers lock");
        let providers = lock.as_mut().expect("unable to get providers");
        providers.push(dilib::global::InjectProvider {
            key: dilib::InjectionKey::with_name::<MyData>("point"),
            provider: dilib::Provider::Scoped(dilib::Scoped::from_construct(
                |container: &dilib::Container| -> MyData {
                    <MyData as dilib::Inject>::inject(container)
                },
            )),
        });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".CRT$XCU"]
    static dilib_MyData_MyData___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn dilib_MyData_MyData___rust_ctor___ctor() {
            dilib_MyData_MyData()
        };
        dilib_MyData_MyData___rust_ctor___ctor
    };
};
struct MyData {
    #[inject(name = "x")]
    x: i32,
    #[inject(name = "y")]
    y: i32,
}
impl dilib::Inject for MyData {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let x: i32 = container
            .get_scoped_with_name("x")
            .expect("cannot get scoped value of name \"x\"");
        let y: i32 = container
            .get_scoped_with_name("y")
            .expect("cannot get scoped value of name \"y\"");
        MyData { x, y }
    }
}
