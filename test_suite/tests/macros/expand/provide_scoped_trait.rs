use dilib::provide;

#[provide(bind="std::ops::Add<usize>")]
fn get_scoped_trait() -> usize {
    123
}