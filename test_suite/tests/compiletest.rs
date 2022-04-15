#[cfg(nightly)]
#[test]
fn main() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/**/*.rs");

    // We are currently running on windows, so this tests break in linux
    #[cfg(target_os = "windows")]
    t.compile_fail("tests/macros/ui/**/*.rs");
}
