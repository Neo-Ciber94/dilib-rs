use dilib::provide;

#[provide]
#[inject(s, data="data")]
fn get_data(s: usize) -> String {
    String::from("")
}

fn main(){}