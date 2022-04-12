use dilib::Inject;
use std::sync::Arc;

trait SomeTrait {
    fn get(&self) -> u32;
}

#[derive(Inject)]
struct MyStruct {
    singleton: Arc<Box<dyn SomeTrait>>
}