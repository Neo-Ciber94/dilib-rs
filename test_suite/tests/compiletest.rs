
#[test]
fn main(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/**/*.rs");
}