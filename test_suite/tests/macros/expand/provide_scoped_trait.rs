use dilib::provide;

#[provide(bind="std::ops::Add<usize>")]
fn get_scoped_trait() -> usize {
    123
}

#[provide(bind="std::ops::Add<usize>, std::ops::Mul<usize>")]
fn get_multiple_scoped_trait() -> usize {
    123
}
