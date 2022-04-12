use dilib::{Inject, provide};
#[cold]
#[doc(hidden)]
#[ctor::ctor]
fn MyStruct__763c899673f142a191b4efe0c0cd50b8() {
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
#[allow(dead_code)]
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
