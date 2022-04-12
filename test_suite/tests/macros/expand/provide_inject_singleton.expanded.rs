use dilib::{Inject, provide};
#[cold]
#[doc(hidden)]
#[ctor::ctor]
fn MyStruct__512f10e655db423fb1dce2fb35ce7312() {
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
