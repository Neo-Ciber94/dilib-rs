use dilib::Inject;
use std::sync::Arc;
trait SomeTrait {
    fn get(&self) -> u32;
}
struct MyStruct {
    singleton: Arc<Box<dyn SomeTrait>>,
}
impl dilib::Inject for MyStruct {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let singleton: dilib::Singleton<Box<dyn SomeTrait>> = container
            .get_singleton()
            .expect("cannot get singleton value of type `Box<dynSomeTrait>`");
        MyStruct { singleton }
    }
}
