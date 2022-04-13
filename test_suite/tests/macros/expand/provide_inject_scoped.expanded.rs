use dilib::{Inject, provide};
const _: () = {
    #[cold]
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[ctor::ctor]
    fn dilib_MyStruct_MyStruct() {
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
};
struct MyStruct {
    x: usize,
    y: Box<String>,
}
impl dilib::Inject for MyStruct {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let x: usize = container
            .get_scoped()
            .expect("cannot get scoped value of type `usize`");
        let y: Box<String> = container
            .get_scoped()
            .expect("cannot get scoped value of type `Box<String>`");
        MyStruct { x, y }
    }
}
