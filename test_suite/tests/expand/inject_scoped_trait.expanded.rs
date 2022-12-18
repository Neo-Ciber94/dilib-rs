use dilib::Inject;
trait SomeTrait {
    fn get(&self) -> u32;
}
struct MyStruct {
    scoped: Box<dyn SomeTrait>,
}
impl dilib::Inject for MyStruct {
    #[allow(unused)]
    #[allow(dead_code)]
    fn inject(container: &dilib::Container) -> Self {
        let scoped: Box<dyn SomeTrait> = container
            .get_scoped()
            .expect("cannot get scoped value of type `Box<dynSomeTrait>`");
        MyStruct { scoped }
    }
}
