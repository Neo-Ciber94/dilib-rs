use dilib::provide;
trait MySingletonTrait {}
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
        providers . push (dilib :: global :: InjectProvider { key : dilib :: InjectionKey :: of :: < std :: boxed :: Box < dyn MySingletonTrait + Send + Sync > > () , provider : dilib :: Provider :: Singleton (dilib :: Shared :: new_lazy (| container : & dilib :: Container | -> std :: boxed :: Box < dyn MySingletonTrait + Send + Sync > { std :: boxed :: Box :: new (< MyStruct as dilib :: Inject > :: inject (container)) })) , }) ;
    }
};
struct MyStruct {
    field: i32,
}
impl MySingletonTrait for MyStruct {}
