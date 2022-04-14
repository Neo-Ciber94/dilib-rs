use dilib::provide;

#[provide(bind="Add<usize>")]
fn get_val() -> usize {
    1234
}

fn main(){}