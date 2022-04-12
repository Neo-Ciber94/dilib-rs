use dilib::Inject;

trait SomeTrait {
    fn get(&self) -> u32;
}

#[derive(Inject)]
struct MyStruct {
    scoped: Box<dyn SomeTrait>
}